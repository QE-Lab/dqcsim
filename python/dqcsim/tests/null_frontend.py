from dqcsim.common import *
from dqcsim.plugin import *
import os

@plugin("Null frontend plugin", "Test", "0.1")
class NullFrontend(Frontend):
    def handle_run(self, *args, **kwargs):
        pass

    def handle_host_work_env(self):
        return ArbData(work=os.getcwd(), env=dict(os.environ))

NullFrontend().run()
