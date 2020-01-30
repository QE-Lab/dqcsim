This more complex example contains Python frontend, operator, and host code.

The frontend is just a quantum algorithm expressed natively using a DQCsim
plugin. It performs all possible 3 + 3 = 4 bit additions at once, by putting
the inputs in superposition and executing a full adder algorithm using CNOT
and (decomposed) Toffoli gates. It then simply observes the output and the
two inputs to look at one of the performed additions.

The operator and host code are more interesting. Together with the QuantumSim
backend, they run whatever quantum-classical algorithm is given as the frontend
for all possible combinations of measurement outcomes of the first N
measurements while recording their probabilities, thus giving the actual
mathematical probabilities versus just running the algorithm 10000 times with
randomized measurements and averaging the results. This works regardless of
any mixed quantum-classical code in the frontend, so you could apply this on
error correction logic too, for example; you can't really do that by executing
an algorithm once and then printing the full qubit state, because the algoritm
depends on the intermediate measurements.

Running the example puts the above two things together to print the
probabilities for the sum of the two random numbers. As a bonus, the algorithm
includes some timing information, and the host code allows you to easily set
the qubit t1/t2 times for QuantumSim to simulate, though the qubits are perfect
by default.
