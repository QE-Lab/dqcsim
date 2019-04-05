
import dqcsim
import threading

scfg = dqcsim.dqcs_scfg_new()
dqcsim.dqcs_scfg_dqcsim_verbosity_set(scfg, dqcsim.DQCS_LOG_TRACE)
dqcsim.dqcs_scfg_stderr_verbosity_set(scfg, dqcsim.DQCS_LOG_TRACE)

front = dqcsim.dqcs_pdef_new(dqcsim.DQCS_PTYPE_FRONT, "test!", "jvs", "3.14")
def kokolores(x, y):
    print("?????? 0x%016X %d" % (x, y))
    dqcs_error_set("???")
    return y
dqcsim.dqcs_pdef_set_run_cb_pyfun(front, kokolores)
front = dqcsim.dqcs_tcfg_new(front, "")
dqcsim.dqcs_scfg_push_plugin(scfg, front)

back = dqcsim.dqcs_pcfg_new(dqcsim.DQCS_PTYPE_BACK, "", "target/debug/dqcsim-plugin")

dqcsim.dqcs_scfg_push_plugin(scfg, back)

sim = dqcsim.dqcs_sim_init(scfg)

arb = dqcsim.dqcs_arb_new()
dqcsim.dqcs_accel_start(sim, arb)
x = dqcsim.dqcs_accel_wait(sim)
print(dqcsim.dqcs_error_get())
print(dqcsim.dqcs_handle_dump(x))

dqcsim.dqcs_handle_delete(sim)
