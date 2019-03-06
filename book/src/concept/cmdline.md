
DQCsim binary command line feature requirements
===============================================
```bash
Usage:

    dqcsim [dqcsim switches] \
        <frontend> [frontend plugin switches] \
        [<operator 1> [operator 1 plugin switches]] \
        [<operator 2> [operator 2 plugin switches]] \
        ...
        [<backend> [backend plugin switches]]
```

DQCsim switches
---------------

 - `--usage`: prints the usage string and exits.
 - `-h`/`--help`: prints a simplified help message and exits.
 - `--longhelp`: prints an extended help message and exits.
 - `-v`/`--version`: prints the version and exits.

### Host API call sequence

DQCsim emulates a simple host program for which the call sequence does not depend on the accelerator output. This sequence is specified by a series of the following switches.  Note that this means that the order matters and that the switches can be repeated.

 - `--start <<arb_data>>`: call `start(arb_data)` with the specified ArbData object (see below for ArbData syntax). Note that `--start '{}'` is automatically called if no `--start` switch is specified, so unless you want to call `start()` more than once or want to pass data to it you don't need to specify it. `wait()` is automatically called before the next `start()` (if any) or the end of the simulation to match the `start()` call.
 - `--send <<arb_data>>`: call `send(arb_data)` with the specified ArbData object (see below for ArbData syntax).
 - `--recv`: call `recv()`. The result is printed in a log message with the "info" level.
 - `--yield`: call `yield()` to explicitly yield to the front-end plugin.
 - `--arb <<arb_cmd>>`: call `arb(arm_cmd)` with the specified ArbCmd object (see below for ArbCmd syntax).

If none of these switches are specified, the default is `--start '{}'`, or, if `--reproduce` is specified, the default is to replay the calls from the reproduction file. If no `--start` is specified but other switches are, `--start '{}'` is added to the front of the list.

### Reproducibility

DQCsim contains a number of features to reproduce an earlier simulation without you having to remember the command line. It's also possible to reproduce a library-based DQCsim run using the command line in most cases.

 - `--repro-out <file.repro>`: output a reproduction file to the specified filename. The default is to output a reproduction file to `<basename(frontend)>.repro`.
 - `--no-repro-out`: disables the reproduction output file.
 - `--repro-absolute`: force all paths printed in the reproduction output file to use absolute paths, even for paths specified relative to the current working directory.
 - `--repro-relative`: force all paths printed in the reproduction output file to use paths relative to the current working directory, even for paths specified relative to the root directory.
 - `--reproduce <file.repro>`: reproduce the simulation run specified by the given reproduction file. If specified in conjunction with host API call sequence switches, full reproduction is performed. If no host API call sequence switches are specified (which would otherwise result in `start(); wait();`), hostless reproduction is performed. It is illegal to combine `--repro-in` with any functional configuration; if you want to change the functional configuration you must change the reproduction file manually.
 - `--reseed`: if specified in conjunction with `--reproduce`, the random seed from the reproduction file is ignored.
 - `--seed <seed>`: specify a random seed for the simulation. A hash of the given seed string is used. If not specified, the current timestamp (with the lowest granularity available) is used as a seed.

### Logging

Log message distribution works as follows:
```bash
                    Source filters                         Output filters
    .----------.         ,-.                            .-.    ,-.
    | Plugin A |---o--->( < )-------------------------->| |--->( < )---> stderr
    '----------'   |     `-'   ,--------------.         | |     `-'
                   |      ^---( plugin A level )        | |      ^    ,------------.
                   |           `--------------'         |B|      '---( stderr level )
                   |     ,-.                            |r|           `------------'
                   o--->( < )---> plugin A --tee file   |o|     ,-.
                   |     `-'   ,------------------.     |a|--->( < )---> DQCsim --tee file
                   :      ^---( plugin --tee level )    |d|     `-'
                   '           `------------------'     |c|      ^    ,--------------------.
    .----------.         ,-.                            |a|      '---( -DQCsim --tee level )
    | Plugin B |---o--->( < )-------------------------->|s|           `--------------------'
    '----------'   |     `-'   ,--------------.         |t|  .
                   :      ^---( plugin B level )        | |  .
                   '           `--------------'         | |  .
    .----------.         ,-.                            | |
    |  DQCsim  |--------( < )-------------------------->| |
    '----------'         `-'   ,------------.           '-'
                          ^---( DQCsim level )
                               `------------'
```
These switches control the production of log messages.

 - `-l <level>`/`--level <level>`: sets the verbosity for the source filters. The plugin source filters can later be overridden. `level` should be `off`/`o`, `error`/`e`, `warn`/`w`, `info`/`i`, `debug`/`d`, or `trace`/`t`. Defaults to `info`.
 - `--plugin-level <level>`: sets the verbosity for the plugin source filters without affecting DQCsim's own verbosity.
 - `--stderr-level <level>`: sets the verbosity for the stderr output filter. Defaults to `trace`, so no output filtering occurs.
 - `--tee <level>:<filename>`: in addition to stderr, also logs to `filename` with the specified output filter verbosity. This switch can be specified multiple times to tee to multiple files.

Plugin specification
--------------------

Plugins are specified using a single string, optionally followed by switches. The string can be any of the following based on context:

 - a valid path to the plugin executable;
 - the basename of the plugin executable with implicit "dqcsfe"/"dqcsop"/"dqcsbe" prefix, searched for in A) the working directory, B) the dqcsim binary directory, and C) the system $PATH;
 - a valid path to a script file with a file extension. In this case, the above rule is run for a plugin named by the file extension of the script file. For instance, if "test.py" is specified for the frontend, DQCsim will look for an executable named "dqcsfepy". The script filename is passed to the plugin using an implicit `--init script.filename.<filename>` switch.

Unless `--reproduce` is active, at least one plugin must be specified, which is treated as the frontend. The backend then defaults to `dqcsbeqx`, the DQCsim backend plugin wrapper for the QX simulator. Specifying a second plugin overrides the default backend. If more plugins are specified, the middle plugins must be operators. If `--reproduce` is active, zero plugins must be specified; the plugin list is taken from the reproduction file instead.

When `--reproduce` is active, most plugin switches become illegal because they are taken from the reproduction file. Logging switches, however, are still well-behaved. In order to indicate to DQCsim that you want to send switches to a previously specified plugin, use the select switch:

 - `--select <name>`: selects a previously defined plugin by name, such that subsequent switches are sent to that plugin.

For example, `dqcsim --reproduce <name.repro> --select front -l debug` will set the loglevel for the frontend to debug, while leaving the loglevels for DQCsim itself and the other plugins set to the default (`info`).

Plugin switches
---------------

Switches specified after a plugin name control that plugin instead of DQCsim as a whole.

### Identification

This switch controls by which name the plugin is referenced. It is illegal to rename plugins, so this switch can only be specified once and only after a plugin definition.

 - `-n`/`--name`: provides a name for the plugin, used for log messages and later `--select` switches. If not provided, plugins are named `front`, `op<i>`, and `back`.

### Plugin creation switches

These switches control how the plugin process is created. This information is part of the reproduction files, so these switches are illegal when `--reproduce` is active.

 - `-i <<arb_cmd>>`/`--init <<arb_cmd>>`: appends an ArbCmd object to the plugin's initialization method.
 - `--env <key>[:<value>]`: sets/updates/overrides the `key` environment variable with an empty string or `value`.
 - `--work <directory>`: overrides the working directory for the plugin.

### Logging

These switches control the production of log messages on a per-plugin basis. These switches can be used even when `--reproduce` is active, since they do not affect the simulation.

 - `-l <level>`/`--level <level>`: overrides the loglevel for the selected plugin. `level` should be `off`/`o`, `error`/`e`, `warn`/`w`, `info`/`i`, `debug`/`d`, or `trace`/`t`.
 - `--stdout <level>`: specifies the loglevel that is to be used for logging the plugin's stdout stream (if any). In addition to the available loglevels (see `-l`), you can also specify `pass`/`p` here, which prevents stdout from being captured by the logging system. The default is `info`.
 - `--stderr <level>`: same as `--stdout`, but for the stderr stream, The default is `info`.
 - `--tee <level>:<filename>`: also logs just the messages for the selected plugin to `filename` with the specified loglevel. This switch can be specified multiple times to tee to multiple files.

ArbData and ArbCmd syntax
-------------------------

ArbData objects can be specified on the command line as follows:
```bash
    <<arb_data>> := '<json>,<arg1>,<arg2>,[...]'
```
`<json>` must be a valid JSON object, surrounded and delimited by `{}`. Zero or more comma-separated strings then follow to specify the unstructured arguments. The following escape characters are available in these argument strings:
```bash
    \,    ->  ,
    \\    ->  \
    \x##  ->  hexadecimal character ##
```
ArbCmd objects are expressed as follows:
```bash
    <<arb_cmd>> := <interface-id>.<operation-id>
                 | <interface-id>.<operation-id>.<arg1>,<arg2>,[...]
                 | <interface-id>.<operation-id>:<<arb_data>>
```
where `<interface-id>` and `<operation-id>` are the interface and operation identifier strings. If `<<arb_data>>` is not specified, it defaults to `{}` with no arguments. If the unstructured data consists of simple strings and the JSON object is not used, the second format can be used, where the JSON object is implicitly `{}`.
