Reproducibility
===============

A DQCsim run is defined by:
 - the reproducible environment:
    - OS
    - filesystem
    - installed packages
    - installed version of DQCsim library
    - docker container
    - environment variables
    - etc.
 - the irreproducible environment:
    - random number generators not governed by the random seed provided by DQCsim
    - system clock
    - OS thread switching
    - etc.
 - the library configuration (as passed to DQCsim's constructor):
    - functional (configuration parameters that affect the quantum/classical algorithm):
       - plugin filenames
       - init arbs passed to each plugin
       - random seed
    - non-functional (configuration parameters that do NOT affect the quantum/classical algorithm, but rather affect the representation of the run):
       - reproduction output file name
       - log filtering
       - ansi coloring/terminal support
       - log callback function
 - the host program/API call sequence (start, wait, send, recv, yield, arb, kill)

By definition, the irreproducible environment cannot be reproduced, and therefore, a DQCsim run cannot necessarily be reproduced. Because reproducibility can be very important for debugging, plugin authors should not depend on irreproducible environment if at all possible. For instance, backend plugins should allow the user to specify a specific random seed, and, if no random seed is specified, the random seed that is actually used should be printed in the form of a log message.

The reproducible environment is by definition reproducible in theory. Specifically, if you run the same simulation twice in a row, and the simulation is correctly configured and written such that it does not depend on anything in the irreproducible environment category, the algorithm should run in exactly the same way, and the output should be exactly the same. Furthermore, changing the non-functional configuration should still not affect the algorithm in any way, but may change the way its output is represented (this is particularly useful for debugging). It may however be difficult to reproduce this environment on a different machine, or after a considerable amount of time has passed (due to OS/package updates for instance). DQCsim does not concern itself with such long-term reproducibility; this is the responsibility of its immediate user, which may be some kind of a driver program in the future.

The functional configuration for DQCsim must be (de)serializable, such that this part of it can be easily reproduced. The non-functional configuration does NOT need to be (de)serializable (it can't be, due to the callback functions dealing with the simulation output), but must be entirely optional, with appropriate defaults substituted for parameters that are not specified.

The DQCsim library should provide a means to output a so-called reproduction file containing the following:

 - the functional library configuration;
 - a log of the host API calls + arguments + return values.

There should be a constructor for the main DQCsim class that takes such a file as input instead of a functional configuration structure for reproduction. This constructor should also take an optional "reseed" argument, which, if set, ignores the recorded random seed and generates a new one. When constructed in this way, the DQCsim class should also provide a `replay()` function that replays the logged API calls, but it's also fine to just rerun the host program if applicable and desirable. In this case and under the following conditions:

 - the reproducible environment was actually reproduced;
 - the plugins and (if applicable) the host program do not depend on the irreproducible environment;
 - the reseed parameter is false or unspecified;

the following should be true:

 - the simulated algorithm runs in exactly the same way, down to the qubit measurements returning the same results (despite the fact that they're random in the real world);
 - the (re)generated reproduction file is identical to the canonical form of the input reproduction file.

Summarizing the above, there are five ways to use DQCsim in terms of reproducibility:

 - the first run: DQCsim is constructed with functional and non-functional configuration structures, then the host program executes some API call sequence.
 - full reproduction: DQCsim is constructed with a reproduction file and a non-functional configuration structure, then the same (reproducible) host program used originally interacts with the simulator. Assuming that the preconditions for reproduction are met, this results in the exact same host and algorithm execution as before. This is primarily intended for debugging (increasing log verbosity, adding debug prints to plugins, etc.).
 - hostless reproduction: DQCsim is constructed with a reproduction file and a non-functional configuration structure, then `replay()` is called. This is the same as above, without requiring availability of the original host program (or, alternatively, requiring that it is reproducible). However, it's more fragile if the host/algorithm communication is complex and one or more plugins are not 100% reproducible. Like above, this is primarily intended for debugging.
 - reproduction with reseeding: DQCsim is constructed with a reproduction file, a non-functional configuration structure, and the `reseed` flag set; then the same (reproducible) host program used originally interacts with the simulator. This is analogous to reproducing a quantum/classical experiment in real life: the things that are not physically reproducible are reseeded. This may for instance be used to incrementally improving the accuracy of some statistical analysis.
 - hostless reproduction with reseeding: DQCsim is constructed with a reproduction file, a non-functional configuration structure, and the `reseed` flag set; then `replay()` is called. Same as above, but only usable when the host-accelerator interaction does not depend on the output of the algorithm.

The reproduction file must be human-readable and (with reasonable effort) human-modifiable to allow for easy design-space exploration by changing just one parameter at a time.

The DQCsim CLI binary should provide the means to generate and replay reproduction files. It should also provide the means to specify the desired API call sequence on the command line, although usually you'd just do `start()` followed by `wait()`, which should be the default.

