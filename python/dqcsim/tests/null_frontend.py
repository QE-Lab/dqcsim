from dqcsim.plugin import *

@plugin("Null frontend plugin", "Test", "0.1")
class NullFrontend(Frontend):
    def handle_run(self, *args, **kwargs):
        pass

NullFrontend().run()
