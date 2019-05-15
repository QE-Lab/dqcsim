# Defining a plugin

Before a plugin can be started, it must be "defined". This process is largely
concerned with installing callback functions and setting some metadata that is
common to all plugins.

## Constructing a plugin definition

Plugin definitions are constructed using `dqcs_pdef_new()`. This function sets
the plugin type and the plugin metadata, which is immutable after the plugin
definition object has been constructed.

@@@c_api_gen ^dqcs_pdef_new$@@@

It is, however, possible to query the metadata and plugin type as follows.

@@@c_api_gen ^dqcs_pdef_type$@@@
@@@c_api_gen ^dqcs_pdef_name$@@@
@@@c_api_gen ^dqcs_pdef_author$@@@
@@@c_api_gen ^dqcs_pdef_version$@@@

## Assigning callback functions

Plugins without callback functions not only don't do anything, they'll crash!
The following matrix shows which functions are required (x), optional (o), and
not applicable (-):

| Callback             | Frontend  | Operator  |  Backend  |
|----------------------|:---------:|:---------:|:---------:|
| `initialize`         |     o     |     o     |     o     |
| `drop`               |     o     |     o     |     o     |
| `run`                |     x     |     -     |     -     |
| `allocate`           |     -     |     o     |     o     |
| `free`               |     -     |     o     |     o     |
| `gate`               |     -     |     o     |     x     |
| `modify_measurement` |     -     |     o     |     -     |
| `advance`            |     -     |     o     |     o     |
| `upstream_arb`       |     -     |     o     |     o     |
| `host_arb`           |     o     |     o     |     o     |

These callback functions can be set using the following functions. Don't forget
to read the general callback information [here](callbacks.apigen.html) as well.

@@@c_api_gen ^dqcs_pdef_set_initialize_cb$@@@
@@@c_api_gen ^dqcs_pdef_set_drop_cb$@@@
@@@c_api_gen ^dqcs_pdef_set_run_cb$@@@
@@@c_api_gen ^dqcs_pdef_set_allocate_cb$@@@
@@@c_api_gen ^dqcs_pdef_set_free_cb$@@@
@@@c_api_gen ^dqcs_pdef_set_gate_cb$@@@
@@@c_api_gen ^dqcs_pdef_set_modify_measurement_cb$@@@
@@@c_api_gen ^dqcs_pdef_set_advance_cb$@@@
@@@c_api_gen ^dqcs_pdef_set_upstream_arb_cb$@@@
@@@c_api_gen ^dqcs_pdef_set_host_arb_cb$@@@
