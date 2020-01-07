from dqcsim.plugin import *

@plugin("Deutsch-Jozsa", "Tutorial", "0.1")
class MyPlugin(Frontend):

    def oracle_constant_0(self, qi, qo):
        """x -> 0 oracle function."""
        pass

    def oracle_constant_1(self, qi, qo):
        """x -> 1 oracle function."""
        self.x_gate(qo)

    def oracle_passthrough(self, qi, qo):
        """x -> x oracle function."""
        self.cnot_gate(qi, qo)

    def oracle_invert(self, qi, qo):
        """x -> !x oracle function."""
        self.cnot_gate(qi, qo)
        self.x_gate(qo)

    def deutsch_jozsa(self, qi, qo, oracle):
        """Runs the Deutsch-Jozsa algorithm on the given oracle. The oracle is
        called with the input and output qubits as positional arguments."""

        # Prepare the input qubit.
        self.prepare(qi)
        self.x_gate(qi)
        self.h_gate(qi)

        # Prepare the output qubit.
        self.prepare(qo)
        self.h_gate(qo)

        # Run the oracle function.
        oracle(qi, qo)

        # Measure the output.
        self.h_gate(qo)
        self.measure(qo)
        if self.get_measurement(qo).value:
            self.info('Oracle was balanced!')
        else:
            self.info('Oracle was constant!')

    def handle_run(self):
        qi, qo = self.allocate(2)

        self.info('Running Deutsch-Jozsa on x -> 0...')
        self.deutsch_jozsa(qi, qo, self.oracle_constant_0)

        self.info('Running Deutsch-Jozsa on x -> 1...')
        self.deutsch_jozsa(qi, qo, self.oracle_constant_1)

        self.info('Running Deutsch-Jozsa on x -> x...')
        self.deutsch_jozsa(qi, qo, self.oracle_passthrough)

        self.info('Running Deutsch-Jozsa on x -> !x...')
        self.deutsch_jozsa(qi, qo, self.oracle_invert)

        self.free(qi, qo)

MyPlugin().run()
