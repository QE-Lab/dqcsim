
import dqcsim

scfg = dqcsim.dqcs_scfg_new()
dqcsim.dqcs_scfg_dqcsim_verbosity_set(scfg, dqcsim.DQCS_LOG_TRACE)
dqcsim.dqcs_scfg_stderr_verbosity_set(scfg, dqcsim.DQCS_LOG_TRACE)
front = dqcsim.dqcs_pcfg_new(dqcsim.DQCS_PTYPE_FRONT, "", "target/debug/dqcsim-plugin")
dqcsim.dqcs_scfg_push_plugin(scfg, front)
back = dqcsim.dqcs_pcfg_new(dqcsim.DQCS_PTYPE_BACK, "", "target/debug/dqcsim-plugin")
dqcsim.dqcs_scfg_push_plugin(scfg, back)
print(dqcsim.dqcs_handle_dump(scfg))
dqcsim.dqcs_accel_init(scfg)
dqcsim.dqcs_accel_drop()
