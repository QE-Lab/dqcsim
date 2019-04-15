"""Contains the base classes for implementing DQCsim plugins.

You should be implementing one of `Frontend`, `Operator`, or `Backend`. The
documentation of these classes indicates which functions you should implement
to specify the plugin's functionality. Overriding `__init__()` is also allowed,
but if you do this you must call `super().__init__()`.

A completed plugin script looks something like this:

    from dqcsim.plugin import Frontend
    from dqcsim.common.arb import ArbData

    class MyPlugin(Frontend):
        def get_name(self):
            return "My Plugin"

        def get_author(self):
            return "Me!"

        def get_version(self):
            return "3.14"

        def handle_run(self, *args, **kwargs):
            # Just return whatever argument we received as the return value!
            return ArbData(*args, **kwargs)

    MyPlugin().run()

This allows the script to be called using DQCsim's command-line interface by
simply specifying the Python script.
"""

__all__ = ['Frontend', 'Operator', 'Backend', 'GateStreamSource', 'Plugin', 'JoinHandle']
__pdoc__ = {
    'JoinHandle.__init__': False
}

import dqcsim._dqcsim as raw
from dqcsim.common import *
import sys
import inspect
import traceback

class JoinHandle(object):
    """Returned by `Plugin.start()` to allow waiting for completion."""
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

    #==========================================================================
    # Launching the plugin
    #==========================================================================
    def __init__(self):
        """Creates the plugin object."""
        self._state_handle = None
        self._started = False

    def _parse_argv(self):
        """Parses argv to get the simulator address."""
        if len(sys.argv) != 2:
            print("Usage: [python3] <script> <simulator-address>", file=sys.stderr)
            print("Note: you should be calling this Python script with DQCsim!", file=sys.stderr)
            sys.exit(1)
        return sys.argv[1]

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

    #==========================================================================
    # API functions operating on plugin state
    #==========================================================================
    def _pc(self, plugin_fn, *args):
        """Use this to call dqcs_plugin functions that take a plugin state."""
        if self._state_handle is None:
            raise RuntimeError("Cannot call plugin operator outside of a callback")
        return plugin_fn(self._state_handle, *args)

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

    #==========================================================================
    # Logging functions
    #==========================================================================
    def _log(self, level, msg, *args, **kwargs):
        # NOTE: we don't need the state handle technically, but this ensures
        # that we're in the right thread.
        if self._state_handle is None:
            raise RuntimeError("Cannot call plugin operator outside of a callback")
        msg = str(msg)
        if args or kwargs:
            msg = msg.format(*args, **kwargs)
        frame = inspect.currentframe().f_back.f_back
        module = frame.f_globals.get('__name__', '?')
        fname = frame.f_globals.get('__file__', '?')
        lineno = frame.f_lineno
        raw.dqcs_log_raw(level, module, fname, lineno, msg)

    def log(self, level, msg, *args, **kwargs):
        """Logs a message with the specified loglevel to DQCsim.

        If any additional positional or keyword arguments are specified, the
        message is formatted using `str.format()`. Otherwise, `str()` is
        applied to the message."""
        # NOTE: this level of indirection is needed to make function name,
        # filename, and line number metadata correct.
        if not isinstance(level, Loglevel):
            raise TypeError('level must be a Loglevel')
        self._log(level, msg, *args, **kwargs)

    def trace(self, msg, *args, **kwargs):
        """Convenience function for logging trace messages. See `log()`."""
        self._log(Loglevel.TRACE, msg, *args, **kwargs)

    def debug(self, msg, *args, **kwargs):
        """Convenience function for logging debug messages. See `log()`."""
        self._log(Loglevel.DEBUG, msg, *args, **kwargs)

    def info(self, msg, *args, **kwargs):
        """Convenience function for logging info messages. See `log()`."""
        self._log(Loglevel.INFO, msg, *args, **kwargs)

    def note(self, msg, *args, **kwargs):
        """Convenience function for logging note messages. See `log()`."""
        self._log(Loglevel.NOTE, msg, *args, **kwargs)

    def warn(self, msg, *args, **kwargs):
        """Convenience function for logging warning messages. See `log()`."""
        self._log(Loglevel.WARN, msg, *args, **kwargs)

    def warning(self, msg, *args, **kwargs):
        """Convenience function for logging warning messages. See `log()`."""
        self._log(Loglevel.WARN, msg, *args, **kwargs)

    def error(self, msg, *args, **kwargs):
        """Convenience function for logging error messages. See `log()`."""
        self._log(Loglevel.ERROR, msg, *args, **kwargs)

    def critical(self, msg, *args, **kwargs):
        """Convenience function for logging fatal messages. See `log()`."""
        self._log(Loglevel.FATAL, msg, *args, **kwargs)

    def fatal(self, msg, *args, **kwargs):
        """Convenience function for logging fatal messages. See `log()`."""
        self._log(Loglevel.FATAL, msg, *args, **kwargs)

    #==========================================================================
    # Callback helpers
    #==========================================================================
    def _cb(self, state_handle, name, *args, **kwargs):
        if hasattr(self, name):
            if self._state_handle is not None:
                raise RuntimeError("Invalid state, recursive callback")
            self._state_handle = state_handle
            try:
                try:
                    return getattr(self, name)(*args, **kwargs)
                except Exception as e:
                    for line in traceback.format_exc().split('\n'):
                        self.trace(line)
                    raise
            finally:
                self._state_handle = None
        raise NotImplementedError("Python plugin doesn't implement {}(), which is a required function!".format(name))

    def _route_converted_arb(self, state_handle, source, cmd):
        if cmd.iface not in self._arb_interfaces.get(source, {}):
            return None
        try:
            return self._cb(state_handle,
                'handle_{}_{}_{}'.format(source, cmd.iface, cmd.oper),
                *cmd._args, **cmd._json
            )
        except NotImplementedError:
            raise ValueError("Invalid operation ID {} for interface ID {}".format(cmd.oper, cmd.iface))

    def _route_initialize(self, state_handle, init_cmds_handle):
        cmds = ArbCmdQueue._from_raw(Handle(init_cmds_handle))
        try:
            self._cb(state_handle, 'handle_init', cmds)
        except NotImplementedError:
            for cmd in cmds:
                self._route_converted_arb(state_handle, 'host', cmd)

    def _route_drop(self, state_handle):
        try:
            self._cb(state_handle, 'handle_drop')
        except NotImplementedError:
            pass

    def _route_host_arb(self, state_handle, cmd_handle):
        result = self._route_converted_arb(state_handle, 'host', ArbCmd._from_raw(Handle(cmd_handle)))
        if result is None:
            result = ArbData()
        if not isinstance(result, ArbData):
            raise TypeError("User implementation of host arb should return None or ArbData but returned {}".format(type(result)))
        return result._to_raw().take()

    def _new_pdef(self, typ):
        pdef = Handle(raw.dqcs_pdef_new(
            typ,
            self._cb(None, 'get_name'),
            self._cb(None, 'get_author'),
            self._cb(None, 'get_version')
        ))
        with pdef as pd:
            raw.dqcs_pdef_set_initialize_cb_pyfun(pd, self._route_initialize)
            raw.dqcs_pdef_set_drop_cb_pyfun(pd, self._route_drop)
            raw.dqcs_pdef_set_host_arb_cb_pyfun(pd, self._route_host_arb)
        return pdef

class GateStreamSource(Plugin):
    """Adds gatestream source functions."""

    #==========================================================================
    # API functions operating on plugin state
    #==========================================================================
    def allocate(self, num_qubits=None, *cmds):
        """Instructs the downstream plugin to allocate one or more qubits.

        If `num_qubits` is specified, this function returns a list of qubit
        references that you can use to refer to the qubits in later function
        calls. These are just integers. If `num_qubits` is not specified or
        `None`, a single qubit is allocated and returned without being wrapped
        in a list.

        Optionally, you can pass (a list of) ArbCmd objects to associate with
        the qubits."""
        with ArbCmdQueue._to_raw(cmds) as cmds:
            qubits = QubitSet._from_raw(Handle(self._pc(
                raw.dqcs_plugin_allocate,
                1 if num_qubits is None else num_qubits,
                cmds
            )))
        if num_qubits is None:
            return qubits[0]
        else:
            return qubits

    def free(self, *qubits):
        """Instructs the downstream plugin to free the given qubits."""
        with QubitSet._to_raw(qubits) as qubits:
            self._pc(raw.dqcs_plugin_free, qubits)

    def unitary(self, targets, matrix, controls=[]):
        """Instructs the downstream plugin to execute a unitary quantum gate.

        `targets` must be a non-empty iterable of qubits or a single qubit,
        representing the qubit(s) targeted by the gate. `matrix` must be a
        unitary matrix appropriately sized for the number of target qubits,
        specified as a row-major one-dimensional list of Python complex
        numbers. `controls` optionally allows additional control qubits to be
        specified to make controlled gates; these qubits should NOT be
        reflected in the gate matrix. The matrix will automatically be extended
        by the downstream plugin, instead. The `targets` and `controls` sets
        must not intersect.
        """
        with QubitSet._to_raw(targets) as targets:
            with QubitSet._to_raw(controls) as controls:
                with Handle(raw.dqcs_gate_new_unitary(targets, controls, matrix)) as gate:
                    self._pc(raw.dqcs_plugin_gate, gate)

    def measure(self, qubits):
        """Instructs the downstream plugin to measure the given qubits in the
        Z basis.

        `qubits` must be an iterable of qubits or a single qubit, representing
        the qubit(s) that are to be measured. If you need to perform a
        measurement in a different basis, either use custom gates or apply the
        appropriate rotations before (and, if necessary, after) the
        measurement.
        """
        with QubitSet._to_raw(qubits) as qubits:
            with Handle(raw.dqcs_gate_new_measurement(qubits)) as gate:
                self._pc(raw.dqcs_plugin_gate, gate)

    def custom_gate(self, name, targets=[], controls=[], measures=[], matrix=None, *args, **kwargs):
        """Instructs the downstream plugin to execute a custom gate.

        `name` must be a non-empty string identifying the gate to be performed.
        `targets`, `constrols`, and `measures` must be iterables of qubits or
        singular qubits, representing respectively the set of qubits to operate
        on, the set of control qubits for controlled gates, and the set of
        qubits measured by the gate.  The `targets` and `controls` sets must
        not intersect. If specified, `matrix` must be a unitary matrix
        appropriately sized for the number of target qubits, specified as a
        row-major one-dimensional list of Python complex numbers. If no matrix
        is applicable or necessary for the custom gate, `None` can be used
        instead. The remainder of the arguments are passed to the constructor
        for `ArbData`; the resulting `ArbData` object is passed along with the
        gate for custom data.
        """
        with QubitSet._to_raw(targets) as targets:
            with QubitSet._to_raw(controls) as controls:
                with QubitSet._to_raw(measures) as measures:
                    gate = Handle(raw.dqcs_gate_new_custom(name, targets, controls, measures, matrix))
                    ArbData(*args, **kwargs)._to_raw(gate)
                    with gate as gate:
                        self._pc(raw.dqcs_plugin_gate, gate)

    def get_measurement(self, qubit):
        """Returns the `Measurement` representing the latest measurement result
        for the given downstream qubit."""
        return Measurement._from_raw(Handle(self._pc(raw.dqcs_plugin_get_measurement, qubit)))

    def get_cycles_since_measure(self, qubit):
        """Returns the number of cycles that have been advanced since the
        latest measurement of the given downstream qubit."""
        return self._pc(raw.dqcs_plugin_get_cycles_since_measure, qubit)

    def get_cycles_between_measures(self, qubit):
        """Returns the number of cycles that were advanced between the
        latest measurement of the given downstream qubit and the one before."""
        return self._pc(raw.dqcs_plugin_get_cycles_between_measures, qubit)

    def advance(self, cycles):
        """Instructs the downstream plugin to advance the simulation time.

        `cycles` must be a nonnegative integer, representing the number of
        cycles to advance the simulation by. The simulation time after the
        advancement is returned."""
        return self._pc(raw.dqcs_plugin_advance, int(cycles))

    def get_cycle(self):
        """Returns the current simulation time for the downstream plugin."""
        return self._pc(raw.dqcs_plugin_get_cycle)

    def arb(self, *args, **kwargs):
        """Sends an `ArbCmd` to the downstream plugin.

        The arguments passed to this function are forwarded directly to the
        `ArbCmd` constructor. The return value is an `ArbData` object
        representing the value returned by the command."""
        with ArbCmd(*args, **kwargs)._to_raw() as cmd:
            return ArbData._from_raw(Handle(self._pc(raw.dqcs_plugin_arb, cmd)))

class Frontend(GateStreamSource):
    """Implements a frontend plugin.

    Frontends execute mixed quantum-classical algorithms, turning them into a
    gatestream for a downstream plugin to consume. They run as slaves to the
    host program, with which they can communicate by means of an ArbData queue
    in either direction.

    The following functions MUST be implemented by the user:

     - `get_name() -> str`

        Must return the name of the plugin implementation.

     - `get_author() -> str`

        Must return the name of the plugin author.

     - `get_version() -> str`

        Must return the plugin's version string.

     - `handle_run(*args, **kwargs) -> ArbData or None`

        Called by the host program through its `start()` API call. The positional
        arguments are set to the list of binary strings from the ArbData
        argument, and **kwargs is set to the JSON object. The returned ArbData
        object can be retrieved by the host using the `wait()` API call. If you
        return None, an empty ArbData object will be automatically generated for
        the response.

    The following functions MAY be implemented by the user:

     - `handle_init(cmds: [ArbCmd]) -> None`

        Called by the simulator to initialize this plugin. The cmds parameter
        is passed a list of ArbCmds that the simulator wishes to associate with
        the plugin. If this function is not implemented, or if it is implemented
        but does not take an argument, the initialization ArbCmds are treated as
        regular host arbs (that is, they're passed to
        `handle_host_<iface>_<oper>()` if those functions do exist).

     - `handle_drop() -> None`

        Called by the simulator when the simulation terminates.

     - `handle_host_<iface>_<oper>(*args, **kwargs) -> ArbData or None`

        Called when an ArbCmd is received from the host with the interface and
        operation identifiers embedded in the name. That is, you don't have to
        do interface/operation identifier matching yourself; you just specify
        the operations that you support. The positional arguments are set to the
        list of binary strings attached to the ArbCmd, and **kwargs is set to
        the JSON object. If you return None, an empty ArbData object will be
        automatically generated for the response.
    """

    #==========================================================================
    # API functions operating on plugin state
    #==========================================================================
    def send(self, *args, **kwargs):
        """Sends an ArbData object to the host.

        The arguments to this function are passed to the constructor of
        `ArbData` to produce the object that is to be sent."""
        data = ArbData(*args, **kwargs)._to_raw()
        with data as d:
            self._pc(raw.dqcs_plugin_send, d)

    def recv(self):
        """Receives an ArbData object to the host.

        This blocks until data is received. The data is returned in the form of
        an `ArbData` object."""
        handle = Handle(self._pc(raw.dqcs_plugin_recv))
        return ArbData._from_raw(handle)

    #==========================================================================
    # Callback helpers
    #==========================================================================
    def _route_run(self, state_handle, arb_handle):
        arg = ArbData._from_raw(Handle(arb_handle))
        result = self._cb(state_handle, 'handle_run', *arg._args, **arg._json)
        if result is None:
            result = ArbData()
        if not isinstance(result, ArbData):
            raise TypeError("User implementation of handle_run() should return None or ArbData but returned {}".format(type(result)))
        return result._to_raw().take()

    def _to_pdef(self):
        """Creates a plugin definition handle for this plugin."""
        pdef = self._new_pdef(raw.DQCS_PTYPE_FRONT)
        with pdef as pd:
            raw.dqcs_pdef_set_run_cb_pyfun(pd, self._route_run)
        return pdef

class Operator(GateStreamSource):
    """Implements an operator plugin.

    Operators sit between frontends and backends, allowing them to observe or
    modify the quantum gate and measurement streams between them.

    The following functions MUST be implemented by the user:

     - `get_name() -> str`

        Must return the name of the plugin implementation.

     - `get_author() -> str`

        Must return the name of the plugin author.

     - `get_version() -> str`

        Must return the plugin's version string.

    The following functions MAY be implemented by the user:

     - `handle_init(cmds: [ArbCmd]) -> None`

        Called by the simulator to initialize this plugin. The cmds parameter
        is passed a list of ArbCmds that the simulator wishes to associate with
        the plugin. If this function is not implemented, or if it is implemented
        but does not take an argument, the initialization ArbCmds are treated as
        regular host arbs (that is, they're passed to
        `handle_host_<iface>_<oper>()` if those functions do exist).

     - `handle_drop() -> None`

        Called by the simulator when the simulation terminates.

     - `handle_allocate(qubits: [Qubit], cmds: [ArbCmd]) -> None`

        Called when the upstream plugin needs more qubits. The qubits list
        specifies the (integer) references that will be used by future calls to
        refer to the qubits (thus, the length of the list is the number of
        qubits that are to be allocated). The cmds parameter is passed a list of
        ArbCmds that the upstream plugin wants to associate with the qubits.

     - `handle_free(qubits: [Qubit]) -> None`

        Called when the upstream plugin doesn't need the specified qubits
        anymore.

     - `handle_unitary_gate(targets: [Qubit], matrix: [complex]) -> None`

        Called when a unitary gate must be handled: it must apply the given
        unitary matrix to the given list of qubits.

     - `handle_controlled_gate(targets: [Qubit], controls: [Qubit], matrix: [complex]) -> None`

        Called when a controlled gate must be handled: it must apply the given
        unitary matrix to the target qubits "if the control qubits are set". In
        other words, it must first turn the given matrix into a controlled
        matrix for the specified number of control qubits, and then apply that
        gate to the concatenation of the target and control lists. If this
        function is not specified, this matrix upscaling is performed
        automatically, allowing `handle_unitary_gate()` to be called instead.
        You only have to implement this if your implementation can get a
        performance boost by doing this conversion manually.

     - `handle_measurement_gate(meas: [Qubit]) -> [Measurement]`

        Called when a measurement must be performed. The measurement basis is
        fixed to the Z-axis; custom gates should be used when different
        measurement bases are required.

        The returned map MAY contain measurement entries for all the qubits
        specified by the qubits parameter, but it is also allowed to not specify
        the measurement results at this time, if an appropriate measurement gate
        is sent downstream and an appropriate `handle_measurement_gate()`
        implementation is provided (or its default is sufficient). This is
        called postponing. Doing this is more performant than reading the
        measurement results of the downstream gate and returning those, because
        it doesn't require waiting for those results to become available.

     -  `handle_<name>_gate(
            targets: [Qubit],
            controls: [Qubit],
            measures: [Qubit],
            matrix: [complex] or None,
            *args, **kwargs
        ) -> {Qubit: value} or None
        `

        Called when a custom (named) gate must be performed. The targets,
        controls, measures, and matrix share the functionality of
        `handle_controlled_gate()` and `handle_measurement_gate()`, as does the
        return value for the latter. Custom gates also have an attached ArbData,
        of which the binary string list is passed to `*args`, and the JSON
        object is passed to `**kwargs`.

     - `handle_measurement(meas: Measurement) -> [measurements]`

        Called when measurement data is received from the downstream plugin,
        allowing it to be modified before it is forwarded upstream. Modification
        includes not passing the measurement through (by returning an empty
        list), turning it into multiple measurements, changing the qubit
        reference to support qubit mapping, or just changing the measurement
        data itself to introduce errors or compensate for an earlier
        modification of the gatestream.

     - `handle_advance(cycles: int) -> None`

        Called to advance simulation time.

     - `handle_<host|upstream>_<iface>_<oper>(*args, **kwargs) -> ArbData or None`

        Called when an ArbCmd is received from the upstream plugin or from the
        host with the interface and operation identifiers embedded in the name.
        That is, you don't have to do interface/operation identifier matching
        yourself; you just specify the operations that you support. The
        positional arguments are set to the list of binary strings attached to
        the ArbCmd, and **kwargs is set to the JSON object. If you return None,
        an empty ArbData object will be automatically generated for the
        response.
    """

    def _to_pdef(self):
        """Creates a plugin definition handle for this plugin."""
        # TODO: callback functions
        raise NotImplementedError()

class Backend(Plugin):
    """Implements a backend plugin.

    Backends consume a quantum gate stream, simulate the gates and qubits, and
    return measurement data to the upstream plugin.

    The following functions MUST be implemented by the user:

     - `get_name() -> str`

        Must return the name of the plugin implementation.

     - `get_author() -> str`

        Must return the name of the plugin author.

     - `get_version() -> str`

        Must return the plugin's version string.

     - `handle_unitary_gate(targets: [Qubit], matrix: [complex]) -> None`

        Called when a unitary gate must be handled: it must apply the given
        unitary matrix to the given list of qubits.

     - `handle_measurement_gate(qubits: [Qubit]) -> [Measurement]`

        Called when a measurement must be performed. The measurement basis is
        fixed to the Z-axis; custom gates should be used when different
        measurement bases are required. The returned list must contain
        measurement data for exactly those qubits specified by the qubits
        parameter.

    The following functions MAY be implemented by the user:

     - `handle_init(cmds: [ArbCmd]) -> None`

        Called by the simulator to initialize this plugin. The cmds parameter
        is passed a list of ArbCmds that the simulator wishes to associate with
        the plugin. If this function is not implemented, or if it is implemented
        but does not take an argument, the initialization ArbCmds are treated as
        regular host arbs (that is, they're passed to
        `handle_host_<iface>_<oper>()` if those functions do exist).

     - `handle_drop() -> None`

        Called by the simulator when the simulation terminates.

     - `handle_allocate(qubits: [Qubit], cmds: [ArbCmd]) -> None`

        Called when the upstream plugin needs more qubits. The qubits list
        specifies the (integer) references that will be used by future calls to
        refer to the qubits (thus, the length of the list is the number of
        qubits that are to be allocated). The cmds parameter is passed a list of
        ArbCmds that the upstream plugin wants to associate with the qubits.

     - `handle_free(qubits: [Qubit]) -> None`

        Called when the upstream plugin doesn't need the specified qubits
        anymore.

     - `handle_controlled_gate(targets: [Qubit], controls: [Qubit], matrix: [complex]) -> None`

        Called when a controlled gate must be handled: it must apply the given
        unitary matrix to the target qubits "if the control qubits are set". In
        other words, it must first turn the given matrix into a controlled
        matrix for the specified number of control qubits, and then apply that
        gate to the concatenation of the target and control lists. If this
        function is not specified, this matrix upscaling is performed
        automatically, allowing `handle_unitary_gate()` to be called instead.
        You only have to implement this if your implementation can get a
        performance boost by doing this conversion manually.

     - `handle_<name>_gate(
            targets: [Qubit],
            controls: [Qubit],
            measures: [Qubit],
            matrix: [complex] or None,
            *args, **kwargs
        ) -> [Measurement]`

        Called when a custom (named) gate must be performed. The targets,
        controls, measures, and matrix share the functionality of
        `handle_controlled_gate()` and `handle_measurement_gate()`, as does the
        return value for the latter. Custom gates also have an attached ArbData,
        of which the binary string list is passed to `*args`, and the JSON
        object is passed to `**kwargs`.

     - `handle_advance(cycles: int) -> None`

        Called to advance simulation time.

     - `handle_<host|upstream>_<iface>_<oper>(*args, **kwargs) -> ArbData or None`

        Called when an ArbCmd is received from the upstream plugin or from the
        host with the interface and operation identifiers embedded in the name.
        That is, you don't have to do interface/operation identifier matching
        yourself; you just specify the operations that you support. The
        positional arguments are set to the list of binary strings attached to
        the ArbCmd, and **kwargs is set to the JSON object. If you return None,
        an empty ArbData object will be automatically generated for the
        response.
    """

    #==========================================================================
    # Callback helpers
    #==========================================================================
    def _route_allocate(self, state_handle, qubits_handle, cmds_handle):
        qubits = QubitSet._from_raw(Handle(qubits_handle))
        cmds = ArbCmdQueue._from_raw(Handle(cmds_handle))
        try:
            self._cb(state_handle, 'handle_allocate', qubits, cmds)
        except NotImplementedError:
            pass

    def _route_free(self, state_handle, qubits_handle):
        qubits = QubitSet._from_raw(Handle(qubits_handle))
        try:
            self._cb(state_handle, 'handle_free', qubits)
        except NotImplementedError:
            pass

    def _route_gate(self, state_handle, gate_handle):
        name = None
        if raw.dqcs_gate_is_custom(gate_handle):
            name = raw.dqcs_gate_name(gate_handle)
        targets = QubitSet._from_raw(Handle(raw.dqcs_gate_targets(gate_handle)))
        controls = QubitSet._from_raw(Handle(raw.dqcs_gate_controls(gate_handle)))
        measures = QubitSet._from_raw(Handle(raw.dqcs_gate_measures(gate_handle)))
        if raw.dqcs_gate_has_matrix(gate_handle):
            matrix = raw.dqcs_gate_matrix(gate_handle)
        else:
            matrix = None

        measurements = []
        if name is None:
            if targets and matrix:
                if controls:
                    try:
                        self._cb(state_handle, 'handle_controlled_gate', targets, controls, matrix)
                    except NotImplementedError:
                        pass

                    # Convert the gate matrix to a controlled gate matrix.
                    cur_nq = len(targets)
                    cur_size = 2**cur_nq
                    assert(len(matrix) == cur_size * cur_size)
                    ext_nq = len(controls) + len(targets)
                    ext_size = 2**ext_nq
                    offset = ext_size - cur_size

                    # Make zero matrix of the right size.
                    ext_matrix = [0.0+0.0j] * (ext_size * ext_size)

                    # Override the lower-right block of the upscaled matrix
                    # with the original matrix.
                    for i in range(cur_size):
                        ext_matrix[(offset + i)*ext_size + offset : (offset + i)*ext_size + ext_size] = matrix[i*cur_size : i*cur_size + cur_size]

                    # Turn the top-left block into an identity matrix.
                    for i in range(offset):
                        ext_matrix[i*ext_size + i] = 1.0+0.0j

                    # Replace the matrix and update the targets.
                    matrix = ext_matrix
                    targets = controls + targets

                self._cb(state_handle, 'handle_unitary_gate', targets, matrix)
            if measures:
                measurements = self._cb(state_handle, 'handle_measurement_gate', measures)
        else:
            data = ArbData._from_raw(Handle(gate_handle))
            measurements = self._cb(state_handle,
                'handle_{}_gate'.format(name),
                targets, controls, measures, matrix, *data._args, **data._json
            )

        return MeasurementSet._to_raw(measurements).take()

    def _route_advance(self, state_handle, cycles):
        try:
            self._cb(state_handle, 'handle_advance', cycles)
        except NotImplementedError:
            pass

    def _route_upstream_arb(self, state_handle, cmd_handle):
        result = self._route_converted_arb(state_handle, 'upstream', ArbCmd._from_raw(Handle(cmd_handle)))
        if result is None:
            result = ArbData()
        if not isinstance(result, ArbData):
            raise TypeError("User implementation of upstream arb should return None or ArbData but returned {}".format(type(result)))
        return result._to_raw().take()

    def _to_pdef(self):
        """Creates a plugin definition handle for this plugin."""
        pdef = self._new_pdef(raw.DQCS_PTYPE_BACK)
        with pdef as pd:
            raw.dqcs_pdef_set_allocate_cb_pyfun(pd, self._route_allocate)
            raw.dqcs_pdef_set_free_cb_pyfun(pd, self._route_free)
            raw.dqcs_pdef_set_gate_cb_pyfun(pd, self._route_gate)
            raw.dqcs_pdef_set_advance_cb_pyfun(pd, self._route_advance)
            raw.dqcs_pdef_set_upstream_arb_cb_pyfun(pd, self._route_upstream_arb)
        return pdef
