
import dqcsim

dqcsim.dqcs_cb_test_install_pyfun(lambda a, b: a + b)
print(dqcsim.dqcs_cb_test_call(3, 5))

dqcsim.dqcs_cb_test_install_pyfun(lambda a, b: a * b)
print(dqcsim.dqcs_cb_test_call(3, 5))

def pyboom(a, b):
    raise RuntimeError('boom!')

dqcsim.dqcs_cb_test_install_pyfun(pyboom)
print(dqcsim.dqcs_cb_test_call(3, 5))
print(dqcsim.dqcs_explain())

def userboom(a, b):
    dqcsim.dqcs_set_error("test test")
    return -1

dqcsim.dqcs_cb_test_install_pyfun(userboom)
print(dqcsim.dqcs_cb_test_call(3, 5))
print(dqcsim.dqcs_explain())

