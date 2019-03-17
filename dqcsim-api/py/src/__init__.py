
__all__ = ['host']

# DQCsim spawns its own threads, so we need to make sure the GIL is
# initialized. From https://stackoverflow.com/a/4870857 it seems that importing
# the threading module ensures this.
import threading
del threading

#import dqcsim.host as host
from dqcsim._dqcsim import *
