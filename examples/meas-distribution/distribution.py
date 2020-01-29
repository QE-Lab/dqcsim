from dqcsim.plugin import *
from dqcsim.host import *

class Distribution(dict):
    """Constructing this class runs a DQCsim simulation with the given
    frontend, (optional) operators, and DQCsim flags for all possible
    combinations of outcomes for the first `num_measurements` measurements
    performed by the frontend/operators, returning the probabilities for each
    possible outcome.

    The positional constructor arguments work the same as they do for
    `dqcsim.host.Simulator()`, except that this simulation runner adds
    its own operator at the back and always uses quantumsim for simulation.
    `num_measurements` limits the number of measurements that are checked
    exhaustively; if not specified, there is no limit. The remaining
    keyword arguments are passed to `dqcsim.host.Simulator()` for logging
    and such.

    The results of the simulation are made available through the dict
    ancestor, with the keys being strings containing all the measurements
    in sequence (for instance '0101' for the first four measurements with
    results 0, 1, 0, and 1) mapping to the probability for that
    sequence.

    The provided frontend and operator plugins must be pure functions
    depending only on the previous measurement results, otherwise the
    search algorithm won't work. You'll need to do monte-carlo sampling in
    that case."""

    @plugin("Distribution", "JvS", "0.1")
    class _DistributionOperator(Operator):
        """Operator used internally to trace the measurement gates and to
        instruct QuantumSim to project to a specific path along the
        tree of possible measurement outcomes."""

        def __init__(self):
            super().__init__()
            self.requested_path = []
            self.followed_path = []

        def handle_measurement_gate(self, qubits, basis, arb):
            for qubit in qubits:
                method = 'probable'
                if self.requested_path:
                    method = self.requested_path.pop(0)
                self.measure(qubit, basis=basis, arb=ArbData(method=method))
                meas = self.get_measurement(qubit)
                self.followed_path.append((meas.value, meas['probability']))

        def handle_host_distribution_prepare(self, requested_path=[]):
            self.requested_path = requested_path

        def handle_host_distribution_retrieve(self):
            path = self.followed_path
            self.followed_path = []
            return ArbData(followed_path=path)

    class _MeasurementChain(object):
        """Internal object to store the measurement outcome tree found so
        far."""

        def __init__(self):
            super().__init__()
            self.meas = None

        def _dump(self, path, trace, depth_remain, dump_intermediate):

            # End of algorithm.
            if self.meas is None:
                yield (path, trace)
                return

            # Reached maximum depth for the dump.
            if not depth_remain:
                yield (path + '...', trace)
                return

            # Dump intermediate results if requested.
            if path and dump_intermediate:
                yield (path + '...', trace)

            # If this measurement is deterministic, dump the deterministic
            # outcome recursively.
            value = self.meas.get_deterministic()
            if value is not None:
                for x in self.meas.result[value]._dump(
                    path + str(value),
                    trace,
                    depth_remain - 1,
                    dump_intermediate
                ):
                    yield x
                return

            # If not, dump both outcomes recursively.
            for value in (0, 1):
                if self.meas.result[value] is None:
                    # Haven't tried this path yet.
                    if depth_remain == 1:
                        yield (path + str(value) + '...', trace * self.meas.p[value])
                    else:
                        yield (path + str(value), None)
                else:
                    # Dump recursively.
                    for x in self.meas.result[value]._dump(
                        path + str(value),
                        trace * self.meas.p[value],
                        depth_remain - 1,
                        dump_intermediate
                    ):
                        yield x

        def dump_raw(self, depth=None, dump_intermediate=False):
            if depth is None:
                depth = -1
            return self._dump('', 1.0, depth, dump_intermediate)

        def dump(self, *args, **kwargs):
            s = []
            for path, probability in self.dump_raw(*args, **kwargs):
                if probability is None:
                    probability = '?'
                else:
                    probability = '%.9f' % probability
                s.append('%s: %s' % (path, probability))
            return '\n'.join(s)

    class _Measurement(object):
        """Internal object to store the measurement outcome tree found so
        far."""

        def __init__(self, p1):
            super().__init__()
            self.p = (1.0 - p1, p1)
            self.result = [None, None]

        def get_deterministic(self):
            if self.p[1] < 1.0e-9:
                return 0
            if self.p[0] < 1.0e-9:
                return 1
            return None

    def _try_path(self, path=()):
        """Runs the simulation for the given "path" of measurement outcomes."""

        self._sim.arb(-2, 'distribution', 'prepare', requested_path=list(path))
        self._sim.run()
        chain = self._chain
        for value, probability in self._sim.arb(-2, 'distribution', 'retrieve')['followed_path']:

            # Get the probability that this measurement is one.
            if value:
                p1 = probability
            else:
                p1 = 1.0 - probability
            p1 = min(max(0.0, p1), 1.0)

            # If we've never done this measurement (with this history) before,
            # construct a new object. Otherwise, check for consistency.
            if chain.meas is None:
                chain.meas = Distribution._Measurement(p1)
            if chain.meas.result[value] is None:
                chain.meas.result[value] = Distribution._MeasurementChain()
            elif abs(chain.meas.p[1] - p1) > 1.e-9:
                raise ValueError('frontend/operators are not pure functions of the measurement results')
            chain = chain.meas.result[value]

    def __init__(self, *plugins, num_measurements=None, quantumsim_kwargs=None, **kwargs):
        super().__init__()

        plugins = list(plugins)
        plugins.append(self._DistributionOperator())
        if quantumsim_kwargs is None:
            quantumsim_kwargs = {}
        plugins.append(('quantumsim', quantumsim_kwargs))
        kwargs['repro'] = None
        self._sim = Simulator(*plugins, **kwargs)
        self._chain = Distribution._MeasurementChain()
        self._num_measurements = num_measurements
        with self._sim:

            # Try the most probable path first.
            self._try_path()

            # Look for a path that still needs simulating.
            while True:
                for path, prob in self._chain.dump_raw(num_measurements):
                    if prob is None:
                        break
                else:
                    break
                self._try_path(map(int, path.replace('.', '')))

        # Populate our dict.
        for path, probability in self._chain.dump_raw(dump_intermediate=True):
            self[path.replace('.', '')] = probability

    def dump(self):
        """Prints a debug dump of the possible outcomes for the first few
        measurements."""
        print(self._chain.dump(self._num_measurements))

if __name__ == '__main__':
    try:
        import dqcsim_quantumsim
    except ImportError:
        import sys
        print('dqcsim-quantumsim module not found, skipping test')
        sys.exit(0)

    dist = Distribution(
        'adder.py',
        num_measurements=4, # <-- remove this to track all possible
        quantumsim_kwargs={     # additions, not just the results
            'init': [
                ArbCmd('quantumsim', 'error', t1=None, t2=None)
            ]                                # ^        ^   set these
        },                                   #  `--------`- to add error!
        stderr_verbosity=Loglevel.INFO
    )                             # ^-- modify to change loglevel

    dist.dump()
