"""Contains wrappers for various DQCsim API handles."""

__all__ = [ #@
    'ArbData',
    'ArbCmd',
    'ArbCmdQueue',
    'Handle',
    'Measurement',
    'MeasurementSet',
    'QubitSet',
    'Loglevel',
]

__pdoc__ = { #@
    # Don't output documentation for objects that the user shouldn't need.
    'cq': False,
    'ArbCmdQueue': False,
    'handle': False,
    'Handle': False,
    'mset': False,
    'MeasurementSet': False,
    'qbset': False,
    'QubitSet': False,

    # Override documentation for the re-exports.
    'ArbData': "Re-export of `dqcsim.common.arb.ArbData`.",
    'ArbCmd': "Re-export of `dqcsim.common.cmd.ArbCmd`.",
    'Measurement': "Re-export of `dqcsim.common.meas.Measurement`.",
}

from enum import IntEnum
import dqcsim._dqcsim as raw

from dqcsim.common.arb import ArbData
from dqcsim.common.cmd import ArbCmd
from dqcsim.common.cq import ArbCmdQueue
from dqcsim.common.handle import Handle
from dqcsim.common.meas import Measurement
from dqcsim.common.mset import MeasurementSet
from dqcsim.common.qbset import QubitSet

class Loglevel(IntEnum):
    """Enumeration of the loglevels available in DQCsim."""

    TRACE = raw.DQCS_LOG_TRACE
    DEBUG = raw.DQCS_LOG_DEBUG
    INFO = raw.DQCS_LOG_INFO
    NOTE = raw.DQCS_LOG_NOTE
    WARN = raw.DQCS_LOG_WARN
    ERROR = raw.DQCS_LOG_ERROR
    FATAL = raw.DQCS_LOG_FATAL
    OFF = raw.DQCS_LOG_OFF

