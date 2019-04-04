
import dqcsim

scfg = dqcsim.dqcs_scfg_new()
dqcsim.dqcs_scfg_dqcsim_verbosity_set(scfg, dqcsim.DQCS_LOG_TRACE)
dqcsim.dqcs_scfg_stderr_verbosity_set(scfg, dqcsim.DQCS_LOG_TRACE)
front = dqcsim.dqcs_pcfg_new(dqcsim.DQCS_PTYPE_FRONT, "", "target/debug/dqcsim-plugin")
dqcsim.dqcs_scfg_push_plugin_process(scfg, front)
back = dqcsim.dqcs_pcfg_new(dqcsim.DQCS_PTYPE_BACK, "", "target/debug/dqcsim-plugin")
dqcsim.dqcs_scfg_push_plugin_process(scfg, back)
sim = dqcsim.dqcs_sim_init(scfg)
dqcsim.dqcs_log_raw(dqcsim.DQCS_LOG_NOTE, "Python!", "???", 3, "hello!")
dqcsim.dqcs_handle_delete(sim)
