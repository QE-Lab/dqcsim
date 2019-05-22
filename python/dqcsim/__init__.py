"""Python API for DQCsim: the Delft Quantum & Classical Simulator

This library allows you to write DQCsim host programs and plugins in Python.
"""

__all__ = ['raw', 'common', 'plugin', 'host']

# Don't try to output documentation for the test module.
__pdoc__ = {'tests': False}

import dqcsim._dqcsim as raw
from dqcsim import common
from dqcsim import plugin
from dqcsim import host
