use crate::arg_parse::plugins::*;
use dqcsim::{
    common::{error::enum_err, log::tee_file::TeeFileConfiguration, log::*, types::*},
    host::{configuration::*, reproduction::*},
};
use std::path::PathBuf;
use structopt::StructOpt;

/// The main StructOpt structure for DQCsim. This encompasses DQCsim's own
/// options.
#[derive(Debug, StructOpt, PartialEq)]
#[structopt(
    name = "DQCsim",
    author = "TU Delft, QuTech",
    about = "Delft Quantum & Classical Simulator"
)]
pub struct DQCsimStructOpt {
    /// Used to specify the host API call sequence. Refer to the "host call
    /// sequence" section for more info.
    #[structopt(
        short = "C",
        long = "call",
        value_name = "call",
        conflicts_with = "reproduce",
        conflicts_with = "reproduce_exactly",
        number_of_values = 1
    )]
    pub host_calls: Vec<HostCall>,

    /// Specifies that the return values of host API calls should be printed to
    /// stdout, in addition to being logged with loglevel note. Use this if you
    /// want to send these values to another program through a pipe.
    #[structopt(long = "host-stdout")]
    pub host_stdout: bool,

    /// Output a reproduction file to the specified filename. The default is
    /// to output a reproduction file to "<basename(frontend)>.repro".
    #[structopt(
        long = "repro-out",
        value_name = "filename",
        conflicts_with = "no_repro_out",
        parse(from_os_str)
    )]
    pub repro_out: Option<PathBuf>,

    /// Disables outputting a reproduction file.
    #[structopt(long = "no-repro-out")]
    pub no_repro_out: bool,

    /// Configures the way paths are stored in the reproduction file. The
    /// default is to save the paths as they were specified on the command
    /// line. The alternatives are to force usage of absolute paths or to force
    /// making them relative to DQCsim's working directory.
    #[structopt(
        long = "repro-paths",
        value_name = "style",
        default_value = "keep",
        case_insensitive = true,
        parse(try_from_str = "parse_enum")
    )]
    pub repro_path_style: ReproductionPathStyle,

    /// Reproduce the simulation run specified by the given reproduction file
    /// as if the modeled physical experiment is rerun. That is, physically
    /// random samples return different values. It is illegal to combine
    /// --reproduce with any functional configuration; if you want to change
    /// the functional configuration you must change the reproduction file
    /// manually.
    #[structopt(
        long = "reproduce",
        value_name = "filename",
        conflicts_with = "reproduce_exactly",
        parse(from_os_str)
    )]
    pub reproduce: Option<PathBuf>,

    /// Reproduce the simulation run specified by the given reproduction file
    /// as exactly as the plugins allow. That is, even physically random
    /// samples should have the same values. It is illegal to combine
    /// --reproduce-exactly with any functional configuration; if you want to
    /// change the functional configuration you must change the reproduction
    /// file manually.
    #[structopt(
        long = "reproduce-exactly",
        value_name = "filename",
        parse(from_os_str)
    )]
    pub reproduce_exactly: Option<PathBuf>,

    /// Specifies a random seed for the simulation. If a 64-bit unsigned number
    /// is specified, it is used directly. Otherwise, the specified string is
    /// hashed to such a 64-bit number. If not specified, the current timestamp
    /// (with the lowest granularity available) is used as a seed.
    #[structopt(
        long = "seed",
        value_name = "seed",
        conflicts_with = "reproduce_exactly",
        parse(from_str)
    )]
    pub seed: Option<Seed>,

    /// Sets the minimum importance for a message to be written to stderr.
    #[structopt(
        short = "l",
        long = "level",
        value_name = "level",
        default_value = "info",
        case_insensitive = true,
        parse(try_from_str = "parse_enum")
    )]
    pub stderr_level: LoglevelFilter,

    /// Logs messages to the specified file in addition to stderr. level sets
    /// the minimum importance for a message to be logged to this file.
    #[structopt(
        short = "T",
        long = "tee",
        value_name = "level>:<filename",
        number_of_values = 1
    )]
    pub tee_files: Vec<TeeFileConfiguration>,

    /// Sets the logging verbosity for DQCsim itself (the driver and host API).
    #[structopt(
        long = "dqcsim-level",
        value_name = "level",
        default_value = "trace",
        case_insensitive = true,
        parse(try_from_str = "parse_enum")
    )]
    pub dqcsim_level: LoglevelFilter,

    /// Sets the default logging verbosity for the plugins.
    #[structopt(
        long = "plugin-level",
        value_name = "level",
        default_value = "trace",
        case_insensitive = true,
        parse(try_from_str = "parse_enum")
    )]
    pub plugin_level: LoglevelFilter,

    /// Shows a more complete help message than --help.
    #[structopt(long = "long-help")]
    pub long_help: bool,
}

/// The plugin StructOpt structure. This encompasses the options that are
/// associated with a plugin.
#[derive(Debug, StructOpt, PartialEq)]
#[structopt()]
pub struct PluginStructOpt {
    /// Provides a custom name for the plugin, used for log messages and
    /// referencing the plugin on the command line later in conjunction with
    /// --reproduce. If not provided, plugins are named "front", "op<i>" (where
    /// i starts at 1 and counts from frontend to backend), and "back".
    #[structopt(short = "n", long = "name", value_name = "name")]
    pub name: Option<String>,

    /// Appends an ArbCmd object to the plugin's initialization method.
    #[structopt(
        short = "i",
        long = "init",
        value_name = "<arb_cmd>",
        number_of_values = 1
    )]
    pub init: Vec<ArbCmd>,

    /// Sets, updates, overrides, or deletes an environment variable in the
    /// plugin scope. To set a variable, use the syntax <key>=<value>. If you
    /// don't care about the value and just want to define the variable, just
    /// <key> without the equals sign is sufficient to assign an empty string.
    /// To delete an environment variable, use ~<key>.
    #[structopt(long = "env", value_name = "mod", number_of_values = 1)]
    pub env: Vec<EnvMod>,

    /// Overrides the working directory for the plugin.
    #[structopt(long = "work", value_name = "filename", parse(from_os_str))]
    pub work: Option<PathBuf>,

    /// Sets the logging verbosity for the associated plugin, overriding
    /// "--plugin-level".
    #[structopt(
        short = "l",
        long = "level",
        value_name = "level",
        case_insensitive = true
    )]
    pub verbosity: Option<LoglevelFilter>,

    /// Logs messages to the specified file in addition to stderr. level sets
    /// the minimum importance for a message to be logged to this file.
    #[structopt(
        short = "T",
        long = "tee",
        value_name = "level>:<filename",
        number_of_values = 1
    )]
    pub tee_files: Vec<TeeFileConfiguration>,

    /// Specifies the loglevel that is to be used for logging the plugin's
    /// stdout stream (if any). In addition to the available loglevels, you
    /// can also specify "pass" here, which prevents stdout from being captured
    /// by the logging system, instead piping it to DQCsim's stdout. The
    /// default is "info".
    #[structopt(long = "stdout", value_name = "level", case_insensitive = true)]
    pub stdout_mode: Option<StreamCaptureMode>,

    /// Specifies the loglevel that is to be used for logging the plugin's
    /// stderr stream (if any). In addition to the available loglevels, you
    /// can also specify "pass" here, which prevents stderr from being captured
    /// by the logging system, instead piping it to DQCsim's stderr. The
    /// default is "info".
    #[structopt(long = "stderr", value_name = "level", case_insensitive = true)]
    pub stderr_mode: Option<StreamCaptureMode>,

    /// Sets the timeout for DQCsim to connect to the plugin after the process
    /// is launched. The default is 5 seconds, so you normally shouldn't have
    /// to touch this. The value accepts floating point numbers as seconds,
    /// integers with time units (h, m, s, ms, us, ns), or "infinity" to
    /// disable the timeout.
    #[structopt(long = "accept-timeout", value_name = "level")]
    pub accept_timeout: Option<Timeout>,

    /// Sets the timeout for plugin shutdown. When this timeout expires, DQCsim
    /// sends SIGKILL to the process to terminate it. The default timeout is 5
    /// seconds, so you normally shouldn't have to touch this. The value
    /// accepts floating point numbers as seconds, integers with time units (h,
    /// m, s, ms, us, ns), or "infinity" to disable the timeout.
    #[structopt(long = "shutdown-timeout", value_name = "level")]
    pub shutdown_timeout: Option<Timeout>,
}

fn parse_enum<E, I>(a: &str) -> Result<E, String>
where
    E: std::str::FromStr
        + strum::IntoEnumIterator<Iterator = I>
        + named_type::NamedType
        + std::fmt::Display,
    I: Iterator<Item = E>,
{
    E::from_str(a).map_err(|_| enum_err::<E, _>(a).to_string())
}

impl From<&PluginStructOpt> for PluginNonfunctionalOpts {
    fn from(opts: &PluginStructOpt) -> Self {
        PluginNonfunctionalOpts {
            verbosity: opts.verbosity,
            tee_files: opts.tee_files.clone(),
            stdout_mode: opts.stdout_mode.clone(),
            stderr_mode: opts.stderr_mode.clone(),
            accept_timeout: opts.accept_timeout,
            shutdown_timeout: opts.shutdown_timeout,
        }
    }
}

impl From<&PluginStructOpt> for PluginProcessFunctionalConfiguration {
    fn from(opts: &PluginStructOpt) -> Self {
        PluginProcessFunctionalConfiguration {
            init: opts.init.clone(),
            env: opts.env.clone(),
            work: opts.work.clone().unwrap_or_else(|| PathBuf::from(".")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_sim_struct() {
        let opt = DQCsimStructOpt {
            host_calls: vec![],
            host_stdout: false,
            repro_out: None,
            no_repro_out: false,
            repro_path_style: ReproductionPathStyle::Keep,
            reproduce: None,
            reproduce_exactly: None,
            seed: None,
            stderr_level: LoglevelFilter::Info,
            tee_files: vec![],
            dqcsim_level: LoglevelFilter::Trace,
            plugin_level: LoglevelFilter::Trace,
            long_help: false,
        };
        assert_eq!(
            opt,
            DQCsimStructOpt::from_clap(&DQCsimStructOpt::clap().get_matches_from(&["test"]))
        );
    }

    #[test]
    fn allow_empty_plugin_opt() {
        assert_eq!(
            PluginStructOpt {
                name: None,
                init: vec![],
                env: vec![],
                work: None,
                verbosity: None,
                tee_files: vec![],
                stdout_mode: None,
                stderr_mode: None,
                accept_timeout: None,
                shutdown_timeout: None,
            },
            PluginStructOpt::from_clap(&PluginStructOpt::clap().get_matches_from(&[""]))
        );
    }

    #[test]
    fn allow_setting_name() {
        let opt = PluginStructOpt {
            name: Some("asdf".to_string()),
            init: vec![],
            env: vec![],
            work: None,
            verbosity: None,
            tee_files: vec![],
            stdout_mode: None,
            stderr_mode: None,
            accept_timeout: None,
            shutdown_timeout: None,
        };
        assert_eq!(
            opt,
            PluginStructOpt::from_clap(
                &PluginStructOpt::clap().get_matches_from(&["test", "--name", "asdf"])
            )
        );
        assert_eq!(
            opt,
            PluginStructOpt::from_clap(
                &PluginStructOpt::clap().get_matches_from(&["test", "-n", "asdf"])
            )
        );
        assert_eq!(
            opt,
            PluginStructOpt::from_clap(
                &PluginStructOpt::clap().get_matches_from(&["test", "-nasdf"])
            )
        );
    }

    #[test]
    fn convert_to_plugin_process_functional_configuration() {
        let opt = PluginStructOpt {
            name: None,
            init: vec![],
            env: vec![],
            work: Some(PathBuf::from(r"/root")),
            verbosity: None,
            tee_files: vec![],
            stdout_mode: None,
            stderr_mode: None,
            accept_timeout: None,
            shutdown_timeout: None,
        };
        let config = PluginProcessFunctionalConfiguration::from(&opt);
        assert_eq!(
            config,
            PluginProcessFunctionalConfiguration {
                init: vec![],
                env: vec![],
                work: PathBuf::from(r"/root")
            }
        );
    }

    #[test]
    fn convert_to_plugin_non_functional_opts() {
        let opt = PluginStructOpt {
            name: None,
            init: vec![],
            env: vec![],
            work: None,
            verbosity: Some(LoglevelFilter::Warn),
            tee_files: vec![],
            stdout_mode: None,
            stderr_mode: None,
            accept_timeout: None,
            shutdown_timeout: None,
        };
        let config = PluginNonfunctionalOpts::from(&opt);
        assert_eq!(
            config,
            PluginNonfunctionalOpts {
                verbosity: Some(LoglevelFilter::Warn),
                tee_files: vec![],
                stdout_mode: None,
                stderr_mode: None,
                accept_timeout: None,
                shutdown_timeout: None
            }
        );
    }

}
