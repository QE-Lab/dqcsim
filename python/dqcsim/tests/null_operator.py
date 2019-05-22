from dqcsim.plugin import *

@plugin("Null operator plugin", "Test", "0.1")
class NullOperator(Operator):
    pass

if __name__ == '__main__':
    NullOperator().run()
