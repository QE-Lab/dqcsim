
__all__ = ['Frontend', 'Operator', 'Backend']

from enum import Enum
import dqcsim._dqcsim as raw
from dqcsim.common import *
import sys


class PluginType(Enum):
    """Represents a plugin type."""
    FRONT = raw.DQCS_PTYPE_FRONT
    OPER = raw.DQCS_PTYPE_OPER
    BACK = raw.DQCS_PTYPE_BACK

class JoinHandle(object):
    def __init__(self, handle):
        if raw.dqcs_handle_type(handle) != raw.DQCS_HTYPE_PLUGIN_JOIN:
            raise TypeError("Specified handle is not a JoinHandle")
        self._handle = handle

    def wait(self):
        """Waits for the associated plugin to finish executing."""
        if self._handle:
            raw.dqcs_plugin_wait(self._handle)
            self._handle.take()

class Plugin(object):
    """Represents a plugin implementation. Must be subclassed; use Frontend,
    Operator, or Backend instead."""

    def __init__(self):
        """Creates the plugin object."""
        self._state_handle = None
        self._started = False

    def _pc(self, plugin_fn, *args):
        """Use this to call dqcs_plugin functions that take a plugin state."""
        if self._state_handle is None:
            raise RuntimeError("Cannot call plugin operator outside of a callback")
        return plugin_fn(self._state_handle, *args)

    def _cb(self, state_handle, fn, *args, **kwargs):
        """Use this to call DQCsim callback functions (defined in this object)
        to store the state handle."""
        if self._state_handle is not None:
            raise RuntimeError("Invalid state, recursive callback")
        self._state_handle = state_handle
        try:
            return fn(*args, **kwargs)
        finally:
            self._state_handle = None

    def random_float(self):
        """Produces a random floating point value between 0 (inclusive) and 1
        (exclusive).

        This function is guaranteed to return the same result every time as
        long as the random seed allocated to us by DQCsim stays the same. This
        allows simulations to be reproduced using a reproduction file. Without
        such a reproduction file or user-set seed, this is of course properly
        (pseudo)randomized."""
        return self._pc(raw.dqcs_plugin_random_f64)

    def random_long(self):
        """Produces a random 64-bit unsigned integer.

        This function is guaranteed to return the same result every time as
        long as the random seed allocated to us by DQCsim stays the same. This
        allows simulations to be reproduced using a reproduction file. Without
        such a reproduction file or user-set seed, this is of course properly
        (pseudo)randomized."""
        return self._pc(raw.dqcs_plugin_random_u64)

    def _parse_argv(self):
        """Parses argv to get the simulator address."""
        if len(sys.argv) != 2:
            print("Usage: [python3] <script> <simulator-address>", file=sys.stderr)
            print("Note: you should be calling this Python script with DQCsim!", file=sys.stderr)
            sys.exit(1)
        return sys.argv[1]

    def _to_pdef(self, cls):
        """Creates a plugin definition handle for this plugin."""
        # TODO: callback functions
        raise NotImplemented()

    def run(self, simulator=None):
        """Instantiates and runs the plugin.

        simulator represents the DQCsim address that the plugin must connect to
        when initializing. It is usually passed as the first argument to the
        plugin process; therefore, if it is not specified, it is taken directly
        from sys.argv."""
        if not hasattr(self, '_state_handle'):
            raise RuntimeError("It looks like you've overwritten __init__ and forgot to call super().__init__(). Please fix!")
        if self._started:
            raise RuntimeError("Plugin has been started before. Make a new instance!")
        if simulator is None:
            simulator = self._parse_argv()
        raw.dqcs_plugin_run(self._to_pdef(), simulator)
        self._started = True

    def start(self, simulator=None):
        """Instantiates and starts the plugin.

        This has the same behavior as run(), except the plugin is started in a
        different thread, so it returns immediately. The returned object is a
        JoinHandle, which contains a wait() method that can be used to wait
        until the plugin finishes executing. Alternatively, if this is not
        done, the plugin thread will (try to) survive past even the main
        thread.

        Note that the JoinHandle can NOT be transferred to a different
        thread!"""
        if not hasattr(self, '_state_handle'):
            raise RuntimeError("It looks like you've overwritten __init__ and forgot to call super().__init__(). Please fix!")
        if self._started:
            raise RuntimeError("Plugin has been started before. Make a new instance!")
        if simulator is None:
            simulator = self._parse_argv()
        handle = Handle(raw.dqcs_plugin_start(self._to_pdef(), simulator))
        self._started = True
        return JoinHandle(handle)

class GateStreamSource(Plugin):
    """Adds gatestream source functions."""

    def allocate(self, num_qubits, *cmds):
        """Instructs the downstream plugin to allocate the given number of
        qubits.

        This function returns a list of qubit references that you can use to
        refer to the qubits in later function calls. These are just integers.

        Optionally, you can pass (a list of) ArbCmd objects to associate with
        the qubits."""
        return QbSet.from_raw(Handle(self._pc(
            raw.dqcs_plugin_allocate,
            num_qubits,
            ArbCmdQueue.to_raw(cq)
        )))

    def free(self, *qubits):
        """Instructs the downstream plugin to free the given qubits."""
        return self._pc(raw.dqcs_plugin_free, QbSet.to_raw(*qubits))

class Frontend(GateStreamSource):
    """Implements a frontend plugin.

    Frontends execute mixed quantum-classical algorithms, turning them into a
    gatestream for a downstream plugin to consume. They run as slaves to the
    host program, with which they can communicate by means of an ArbData queue
    in either direction.

    The following functions MUST be overridden by the user:

     - get_name() -> PluginType
       Must return the name of the plugin implementation.

     - get_author() -> PluginType
       Must return the name of the plugin author.

     - get_version() -> PluginType
       Must return the plugin's version string.

     - handle_run(*args, **kwargs) -> ArbData or None
       Called by the host program through its start() API call. The positional
       arguments are set to the list of binary strings from the ArbData
       argument, and **kwargs is set to the JSON object. The returned ArbData
       object can be retrieved by the host using the wait() API call. If you
       return None, an empty ArbData object will be automatically generated for
       the response.

    The following functions MAY be overridden by the user:

     - handle_init(cmds: [ArbCmd]) -> None
       Called by the simulator to initialize this plugin. The cmds parameter
       is passed a list of ArbCmds that the simulator wishes to associate with
       the plugin. If this function is not implemented, or if it is implemented
       but does not take an argument, the initialization ArbCmds are treated as
       regular host arbs (that is, they're passed to
       handle_host_<iface>_<oper>() if those functions do exist).

     - handle_drop() -> None
       Called by the simulator when the simulation terminates.

     - handle_host_<iface>_<oper>(*args, **kwargs) -> ArbData or None
       Called when an ArbCmd is received from the host with the interface and
       operation identifiers embedded in the name. That is, you don't have to
       do interface/operation identifier matching yourself; you just specify
       the operations that you support. The positional arguments are set to the
       list of binary strings attached to the ArbCmd, and **kwargs is set to
       the JSON object. If you return None, an empty ArbData object will be
       automatically generated for the response.
    """

    def get_type():
        """Returns that this is a frontend plugin."""
        return PluginType.FRONT

class Operator(GateStreamSource):
    """Implements an operator plugin.

    Operators sit between frontends and backends, allowing them to observe or
    modify the quantum gate and measurement streams between them.

    The following functions MUST be overridden by the user:

     - get_name() -> PluginType
       Must return the name of the plugin implementation.

     - get_author() -> PluginType
       Must return the name of the plugin author.

     - get_version() -> PluginType
       Must return the plugin's version string.

    The following functions MAY be overridden by the user:

     - handle_init(cmds: [ArbCmd]) -> None
       Called by the simulator to initialize this plugin. The cmds parameter
       is passed a list of ArbCmds that the simulator wishes to associate with
       the plugin. If this function is not implemented, or if it is implemented
       but does not take an argument, the initialization ArbCmds are treated as
       regular host arbs (that is, they're passed to
       handle_host_<iface>_<oper>() if those functions do exist).

     - handle_drop() -> None
       Called by the simulator when the simulation terminates.

     - handle_allocate(qubits: [Qubit], cmds: [ArbCmd]) -> None
       Called when the upstream plugin needs more qubits. The qubits list
       specifies the (integer) references that will be used by future calls to
       refer to the qubits (thus, the length of the list is the number of
       qubits that are to be allocated). The cmds parameter is passed a list of
       ArbCmds that the upstream plugin wants to associate with the qubits.

     - handle_free(qubits: [Qubit]) -> None
       Called when the upstream plugin doesn't need the specified qubits
       anymore.

     - handle_unitary_gate(targets: [Qubit], matrix: [[complex]]) -> None
       Called when a unitary gate must be handled: it must apply the given
       unitary matrix to the given list of qubits.

     - handle_controlled_gate(targets: [Qubit], controls: [Qubit], matrix: [[complex]]) -> None
       Called when a controlled gate must be handled: it must apply the given
       unitary matrix to the target qubits "if the control qubits are set". In
       other words, it must first turn the given matrix into a controlled
       matrix for the specified number of control qubits, and then apply that
       gate to the concatenation of the target and control lists. If this
       function is not specified, this matrix upscaling is performed
       automatically, allowing handle_unitary_gate() to be called instead.
       You only have to implement this if your implementation can get a
       performance boost by doing this conversion manually.

     - handle_measurement_gate(meas: [Qubit]) -> [Measurement]
       Called when a measurement must be performed. The measurement basis is
       fixed to the Z-axis; custom gates should be used when different
       measurement bases are required.

       The returned map MAY contain measurement entries for all the qubits
       specified by the qubits parameter, but it is also allowed to not specify
       the measurement results at this time, if an appropriate measurement gate
       is sent downstream and an appropriate handle_measurement_gate()
       implementation is provided (or its default is sufficient). This is
       called postponing. Doing this is more performant than reading the
       measurement results of the downstream gate and returning those, because
       it doesn't require waiting for those results to become available.

     - handle_<name>_gate(
         targets: [Qubit],
         controls: [Qubit],
         measures: [Qubit],
         matrix: [[complex]] or None,
         *args, **kwargs
       ) -> {Qubit: value} or None
       Called when a custom (named) gate must be performed. The targets,
       controls, measures, and matrix share the functionality of
       handle_controlled_gate() and handle_measurement_gate(), as does the
       return value for the latter. Custom gates also have an attached ArbData,
       of which the binary string list is passed to *args, and the JSON object
       is passed to **kwargs.

     - handle_measurement(meas: Measurement) -> [measurements]
       Called when measurement data is received from the downstream plugin,
       allowing it to be modified before it is forwarded upstream. Modification
       includes not passing the measurement through (by returning an empty
       list), turning it into multiple measurements, changing the qubit
       reference to support qubit mapping, or just changing the measurement
       data itself to introduce errors or compensate for an earlier
       modification of the gatestream.

     - handle_advance(cycles: [int]) -> None
       Called to advance simulation time.

     - handle_<host|upstream>_<iface>_<oper>(*args, **kwargs) -> ArbData or None
       Called when an ArbCmd is received from the upstream plugin or from the
       host with the interface and operation identifiers embedded in the name.
       That is, you don't have to do interface/operation identifier matching
       yourself; you just specify the operations that you support. The
       positional arguments are set to the list of binary strings attached to
       the ArbCmd, and **kwargs is set to the JSON object. If you return None,
       an empty ArbData object will be automatically generated for the
       response.
    """

    def get_type():
        """Returns that this is an operator plugin."""
        return PluginType.OPER

class Backend(Plugin):
    """Implements a backend plugin.

    Backends consume a quantum gate stream, simulate the gates and qubits, and
    return measurement data to the upstream plugin.

    The following functions MUST be overridden by the user:

     - get_name() -> PluginType
       Must return the name of the plugin implementation.

     - get_author() -> PluginType
       Must return the name of the plugin author.

     - get_version() -> PluginType
       Must return the plugin's version string.

     - handle_unitary_gate(targets: [Qubit], matrix: [[complex]]) -> None
       Called when a unitary gate must be handled: it must apply the given
       unitary matrix to the given list of qubits.

     - handle_measurement_gate(qubits: [Qubit]) -> [Measurement]
       Called when a measurement must be performed. The measurement basis is
       fixed to the Z-axis; custom gates should be used when different
       measurement bases are required. The returned list must contain
       measurement data for exactly those qubits specified by the qubits
       parameter.

    The following functions MAY be overridden by the user:

     - handle_init(cmds: [ArbCmd]) -> None
       Called by the simulator to initialize this plugin. The cmds parameter
       is passed a list of ArbCmds that the simulator wishes to associate with
       the plugin. If this function is not implemented, or if it is implemented
       but does not take an argument, the initialization ArbCmds are treated as
       regular host arbs (that is, they're passed to
       handle_host_<iface>_<oper>() if those functions do exist).

     - handle_drop() -> None
       Called by the simulator when the simulation terminates.

     - handle_allocate(qubits: [Qubit], cmds: [ArbCmd]) -> None
       Called when the upstream plugin needs more qubits. The qubits list
       specifies the (integer) references that will be used by future calls to
       refer to the qubits (thus, the length of the list is the number of
       qubits that are to be allocated). The cmds parameter is passed a list of
       ArbCmds that the upstream plugin wants to associate with the qubits.

     - handle_free(qubits: [Qubit]) -> None
       Called when the upstream plugin doesn't need the specified qubits
       anymore.

     - handle_controlled_gate(targets: [Qubit], controls: [Qubit], matrix: [[complex]]) -> None
       Called when a controlled gate must be handled: it must apply the given
       unitary matrix to the target qubits "if the control qubits are set". In
       other words, it must first turn the given matrix into a controlled
       matrix for the specified number of control qubits, and then apply that
       gate to the concatenation of the target and control lists. If this
       function is not specified, this matrix upscaling is performed
       automatically, allowing handle_unitary_gate() to be called instead.
       You only have to implement this if your implementation can get a
       performance boost by doing this conversion manually.

     - handle_<name>_gate(
         targets: [Qubit],
         controls: [Qubit],
         measures: [Qubit],
         matrix: [[complex]] or None,
         *args, **kwargs
       ) -> [Measurement]
       Called when a custom (named) gate must be performed. The targets,
       controls, measures, and matrix share the functionality of
       handle_controlled_gate() and handle_measurement_gate(), as does the
       return value for the latter. Custom gates also have an attached ArbData,
       of which the binary string list is passed to *args, and the JSON object
       is passed to **kwargs.

     - handle_advance(cycles: [int]) -> None
       Called to advance simulation time.

     - handle_<host|upstream>_<iface>_<oper>(*args, **kwargs) -> ArbData or None
       Called when an ArbCmd is received from the upstream plugin or from the
       host with the interface and operation identifiers embedded in the name.
       That is, you don't have to do interface/operation identifier matching
       yourself; you just specify the operations that you support. The
       positional arguments are set to the list of binary strings attached to
       the ArbCmd, and **kwargs is set to the JSON object. If you return None,
       an empty ArbData object will be automatically generated for the
       response.
    """

    def get_type():
        """Returns that this is a backend plugin."""
        return PluginType.BACK
