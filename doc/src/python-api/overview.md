# Overview

To use DQCsim from Python, simply import the `dqcsim` module:

```python
import dqcsim
```

If you don't have
it yet, you can install it using `pip3 install dqcsim` (more detailed notes
[here](../install/index.html)).

The `dqcsim` module is divided up into four public submodules:

 - `dqcsim.common` contains wrappers for `ArbData` and `ArbCmd` objects, as
   well as measurements.
 - `dqcsim.plugin` contains base classes for the three different plugin types.
   To implement a plugin, you just have to derive from one of them.
 - `dqcsim.host` contains a wrapper for DQCsim as a whole, allowing you to use
   the simulation as a "quantum accelerator" within your code.
 - `dqcsim.tests` contains `nose` tests for all of the above.
