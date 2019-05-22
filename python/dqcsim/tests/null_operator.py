from dqcsim.plugin import *

@plugin("Null operator plugin", "Test", "0.1")
class NullOperator(Operator):
    pass

NullOperator().run()
