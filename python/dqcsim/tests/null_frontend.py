from dqcsim.common import *
from dqcsim.plugin import *
import os

@plugin("Null frontend plugin", "Test", "0.1")
class NullFrontend(Frontend):
    def __init__(self, *args, **kwargs):
        super().__init__()
        self.arbs_received = []

    def handle_drop(self):
        self.info('info message')
        self.trace('null frontend dropped!')

    def handle_run(self, *args, **kwargs):
        pass

    def handle_host_work_env(self):
        return ArbData(work=os.getcwd(), env=dict(os.environ))

    def handle_host_x_y(self, *args, **kwargs):
        self.arbs_received.append({'iface': 'x', 'oper': 'y', 'args': args, 'kwargs': kwargs})

    def handle_host_y_z(self, *args, **kwargs):
        self.arbs_received.append({'iface': 'y', 'oper': 'z', 'args': args, 'kwargs': kwargs})

    def handle_host_get_arbs(self):
        return ArbData(data=self.arbs_received)

if __name__ == '__main__':
    NullFrontend().run()
