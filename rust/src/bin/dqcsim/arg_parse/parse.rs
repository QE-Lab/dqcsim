use crate::arg_parse::{opts::*, plugins::*};
use ansi_term::Colour;
use dqcsim::{
    common::types::*,
    host::configuration::PluginConfiguration,
    host::{configuration::*, reproduction::*},
};
use failure::{Error, Fail};
use git_testament::{git_testament, render_testament};
use std::{ffi::OsString, path::PathBuf, str::FromStr};
use structopt::{clap::AppSettings, StructOpt};

/// Error structure used for reporting command line errors.
///
/// The messages contained within contain ANSI formatting and are intended to
/// be returned to the user.
#[derive(Debug, Fail, PartialEq)]
pub enum CommandLineError {
    #[fail(display = "{}", 0)]
    Unknown(String),
    #[fail(display = "{}", 0)]
    LongHelp(String),
}

/// Utility struct used to parse the plugin part of the command line. Used like
/// this:
///
/// ```rust
/// let mut parser = PluginConfigParser::new(plugin_clap_app);
/// parser.parse(&dqcsim_matches);
/// if reproducing {
///     let mods = parser.get_mods();
/// } else {
///     let defs = parser.get_defs();
/// }
/// ```
struct PluginConfigParser<'a, 'b> {
    app: clap::App<'a, 'b>,
    defs: Vec<PluginDefinition>,
    mods: Vec<PluginModification>,
    first_specification: Option<String>,
}

impl<'a, 'b> PluginConfigParser<'a, 'b> {
    /// Constructs a new parser object from the plugin clap app.
    pub fn new(app: clap::App<'a, 'b>) -> PluginConfigParser<'a, 'b> {
        PluginConfigParser {
            app,
            defs: vec![],
            mods: vec![],
            first_specification: None,
        }
    }

    /// Parse the subcommand (if any) embedded in `prev_matches` as a plugin
    /// configuration.
    ///
    /// If `prev_matches` does not contain a subcommand, this function is
    /// no-op. If it does, it is parsed as a plugin configuration, and is
    /// pushed into `self.defs` or `self.mods` depending on the subcommand
    /// syntax. This function recurses if the plugin configuration itself
    /// contains a subcommand, so this function can parse zero or more plugins.
    pub fn parse(&mut self, prev_matches: &clap::ArgMatches) -> Result<(), Error> {
        // See if prev_matches contains a subcommand.
        if let (specification, Some(unparsed_matches)) = prev_matches.subcommand() {
            self.parse_from(
                specification,
                unparsed_matches.values_of("").unwrap_or_default(),
            )
        } else {
            Ok(())
        }
    }

    /// Add a plugin, given its specification string and a list of arguments.
    ///
    /// This recursively calls `parse()` on the parsed `clap::ArgMatches`
    /// object.
    fn parse_from<I, T>(&mut self, specification: &str, args: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        // Parse the plugin options using clap and structopt.
        let matches = self.app.clone().get_matches_from(args);
        let opts = PluginStructOpt::from_clap(&matches);

        // Determine the plugin type.
        let plugin_type;
        if self.defs.is_empty() {
            plugin_type = PluginType::Frontend;
        } else if matches.subcommand_name().is_some() {
            plugin_type = PluginType::Operator;
        } else {
            plugin_type = PluginType::Backend;
        }

        // Switch based on whether this is a definition or a modification.
        if specification.starts_with('@') {
            // Modifications do not allow renaming the plugin or changing
            // functional arguments. So, report an error if any of these
            // options are present.
            for ill_switch in &["name", "init", "env", "work"] {
                if matches.is_present(ill_switch) {
                    return format_error(CommandLineError::Unknown(format!(
                        "The argument '{}' cannot be used when referencing a previously defined plugin",
                        Colour::Yellow.paint(format!("--{}", ill_switch))
                    )));
                }
            }

            // Push the plugin modification.
            let opts = PluginNonfunctionalOpts::from(&opts);
            self.mods.push(PluginModification {
                name: specification[1..].to_string(),
                verbosity: opts.verbosity,
                tee_files: opts.tee_files,
                stdout_mode: opts.stdout_mode,
                stderr_mode: opts.stderr_mode,
                accept_timeout: opts.accept_timeout,
                shutdown_timeout: opts.shutdown_timeout,
            });
        } else {
            // Figure out a default name for the plugin based on the type.
            let default_name = match plugin_type {
                PluginType::Frontend => "front".to_string(),
                PluginType::Operator => format!("op{}", self.defs.len()),
                PluginType::Backend => "back".to_string(),
            };

            // If this is the first plugin definition, save the sugared
            // specification. This is used to construct the default name for
            // the reproduction output file.
            if self.first_specification.is_none() {
                self.first_specification.replace(specification.to_string());
            }

            // Push the plugin definition.
            self.defs.push(PluginDefinition {
                name: opts.name.clone().unwrap_or(default_name),
                specification: PluginProcessSpecification::from_sugar(specification, plugin_type)
                    .or_else(|e| {
                    format_error_ctxt("While interpreting plugin specification", e)
                })?,
                functional: (&opts).into(),
                nonfunctional: (&opts).into(),
            });
        }

        self.parse(&matches)
    }

    /// Returns the vector of plugin definitions.
    ///
    /// When DQCsim is running in reproduction mode, use `get_mods()` instead.
    /// If zero plugins were defined or any plugin modifications were
    /// specified, an error is reported. If only one plugin is defined, QX is
    /// appended with the default configuration.
    pub fn get_defs(mut self) -> Result<(PathBuf, Vec<PluginDefinition>), Error> {
        if self.defs.is_empty() {
            return format_error(CommandLineError::Unknown(
                "At least one plugin specification is required".to_string(),
            ));
        }

        // Add the default backend if only the frontend is specified.
        if self.defs.len() == 1 {
            let empty: Vec<OsString> = vec![];
            self.parse_from("qx", empty)?;
        }

        // Apply any mods.
        // FIXME: mods currently don't play nice with defs because specifying a
        // mod after the last def turns the last def into an operator instead
        // of a backend. This is because it sees a subcommand behind it and
        // assumes there is at least one more definition.
        //for m in self.mods {
        //    m.apply(&mut self.defs).unwrap_or_else(|e| error(e.to_string()));
        //}
        if !self.mods.is_empty() {
            return format_error(CommandLineError::Unknown(format!(
                "Cannot modify plugins unless '{}' or '{}' is active",
                Colour::Green.paint("--reproduce"),
                Colour::Green.paint("--reproduce-exactly")
            )));
        }

        Ok((self.first_specification.unwrap().into(), self.defs))
    }

    /// Returns the vector of plugin modifications.
    ///
    /// Use when DQCsim is running in reproduction mode. If any plugins were
    /// defined, an error is reported.
    pub fn get_mods(self) -> Result<Vec<PluginModification>, Error> {
        if !self.defs.is_empty() {
            return format_error(CommandLineError::Unknown(format!(
                "Cannot define new plugins while '{}' or '{}' is active",
                Colour::Green.paint("--reproduce"),
                Colour::Green.paint("--reproduce-exactly")
            )));
        }

        Ok(self.mods)
    }
}

/// The complete configuration for a DQCsim run.
#[derive(Debug)]
pub struct CommandLineConfiguration {
    /// The sequence of host calls to make.
    ///
    /// Note that `wait()` is not represented in the `HostCall` enumeration.
    /// `wait()` calls should instead be inserted automatically as late as
    /// possible, that is:
    ///
    ///  - when `HostCall::Start` is encountered while the accelerator was
    ///    already started;
    ///  - before DQCsim terminates, if the accelerator is still running.
    pub host_calls: Vec<HostCall>,

    /// Specifies that the return values of host API calls should be printed to
    /// stdout, in addition to being logged with loglevel note.
    pub host_stdout: bool,

    /// The simulator configuration.
    pub dqcsim: SimulatorConfiguration,

    /// Reproduction output filename.
    pub reproduction_file: Option<PathBuf>,
}

git_testament!(TESTAMENT);

impl CommandLineConfiguration {
    /// Produces a DQCsim configuration from the specified command line
    /// argument iterable.
    ///
    /// A reproduction file is loaded in addition if `--reproduce` or
    /// `--reproduce-exactly` was specified, and a reproduction file is also
    /// written through `write_repro()` if requested.
    ///
    /// The returned error contains ANSI formatting and is intended to be
    /// printed directly to the user. Among other things, it may contain the
    /// help message.
    pub fn parse_from<I, T>(args: I) -> Result<CommandLineConfiguration, Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        // Generate the version string.
        let version = render_testament!(TESTAMENT);

        // Generate the ASCII art.
        let ascii_art = format!(
            include_str!("ascii.txt"),
            format!("Version {}", version),
            format!("Built {}", env!("COMPILED_AT"))
        );

        // Generate the plugin-specific option parser. It's still mutable here
        // because write_long_help() requires mutability.
        let mut plugin_clap = PluginStructOpt::clap()
            .settings(&[
                AppSettings::AllowExternalSubcommands,
                AppSettings::DeriveDisplayOrder,
                AppSettings::UnifiedHelpMessage,
                AppSettings::NextLineHelp,
                AppSettings::ColoredHelp,
                AppSettings::DisableVersion,
                AppSettings::NoBinaryName,
            ])
            .template("PLUGIN OPTIONS:\n{unified}");

        // Extract the generated help string from the plugin-specific parser, so we
        // can add it to the generated help of DQCsim as a whole.
        let mut plugin_help: Vec<u8> = vec![];
        plugin_clap.write_long_help(&mut plugin_help).unwrap();
        let plugin_clap = plugin_clap;
        let plugin_help = String::from_utf8(plugin_help).unwrap();

        // Generate the option parser for DQCsim's own options.
        let dqcsim_clap = DQCsimStructOpt::clap()
            .settings(&[
                AppSettings::AllowExternalSubcommands,
                AppSettings::DeriveDisplayOrder,
                AppSettings::UnifiedHelpMessage,
                AppSettings::NextLineHelp,
                AppSettings::ColoredHelp,
            ])
            .version(version.as_ref())
            .usage(include_str!("usage.txt").trim_end())
            .template(include_str!("short-help-template.txt").trim_end())
            .before_help(version.as_ref())
            .after_help(plugin_help.as_ref());

        // Parse DQCsim's options.
        let dqcsim_matches = dqcsim_clap.clone().get_matches_from_safe(args)?;
        let dqcsim_opts = DQCsimStructOpt::from_clap(&dqcsim_matches);

        // Handle the --long-help switch.
        if dqcsim_opts.long_help {
            let mut dqcsim_clap = dqcsim_clap
                .template(include_str!("long-help-template.txt").trim_end())
                .before_help(ascii_art.trim_end().as_ref());
            let mut long_help: Vec<u8> = vec![];
            dqcsim_clap.write_long_help(&mut long_help).unwrap();
            let long_help = String::from_utf8(long_help).unwrap();
            return Err(CommandLineError::LongHelp(long_help).into());
        }

        // Parse the plugin options.
        let mut pcp = PluginConfigParser::new(plugin_clap);
        pcp.parse(&dqcsim_matches)?;

        // Build the DQCsim configuration structure.
        let mut config = CommandLineConfiguration {
            host_calls: vec![],
            host_stdout: dqcsim_opts.host_stdout,
            dqcsim: SimulatorConfiguration {
                seed: dqcsim_opts.seed.clone().unwrap_or_default(),
                stderr_level: dqcsim_opts.stderr_level,
                tee_files: dqcsim_opts.tee_files.clone(),
                log_callback: None,
                dqcsim_level: dqcsim_opts.dqcsim_level,
                plugins: vec![],
                reproduction_path_style: if dqcsim_opts.no_repro_out {
                    None
                } else {
                    Some(dqcsim_opts.repro_path_style)
                },
            },
            reproduction_file: dqcsim_opts.repro_out.clone(),
        };

        // Configure the plugins and handle the reconfiguration options.
        if dqcsim_opts.reproduce.is_some() || dqcsim_opts.reproduce_exactly.is_some() {
            let plugin_mods = pcp.get_mods()?;
            let exact = dqcsim_opts.reproduce_exactly.is_some();

            // Get the configuration file filename.
            let file = dqcsim_opts
                .reproduce
                .as_ref()
                .or_else(|| dqcsim_opts.reproduce_exactly.as_ref())
                .unwrap();

            // Parse the reproduction file and update the configuration with
            // it.
            config.host_calls = Reproduction::from_file(file)
                .or_else(|e| format_error_ctxt("While reading reproduction file", e))?
                .to_run(&mut config.dqcsim, plugin_mods, exact)
                .or_else(|e| format_error_ctxt("While loading reproduction file", e))?;
        } else {
            // Construct the plugin vector from the plugin definitions.
            let (first_specification, defs) = pcp.get_defs()?;
            config.dqcsim.plugins = defs
                .into_iter()
                .map(|x| {
                    Box::new(x.into_config(dqcsim_opts.plugin_level))
                        as Box<dyn PluginConfiguration>
                })
                .collect();

            // If the user did not explicitly request a start() host call, add
            // one to the front of the list.
            let mut running = if !dqcsim_opts.host_calls.iter().any(|x| match x {
                HostCall::Start(_) => true,
                _ => false,
            }) {
                config.host_calls.push(HostCall::Start(ArbData::default()));
                true
            } else {
                false
            };

            // Populate the rest of the call list, inserting wait() calls as
            // late as possible when needed.
            for host_call in dqcsim_opts.host_calls.iter() {
                match host_call {
                    HostCall::Start(_) => {
                        if running {
                            config.host_calls.push(HostCall::Wait);
                        }
                        running = true;
                    }
                    HostCall::Wait => {
                        running = false;
                    }
                    _ => (),
                }
                config.host_calls.push(host_call.clone());
            }
            if running {
                config.host_calls.push(HostCall::Wait);
            }

            // Even if the user did not specify a reproduction output file, the
            // default behavior is to output one for new simulations.
            if config.reproduction_file.is_none() {
                config.reproduction_file.replace(
                    PathBuf::from_str(&format!(
                        "{}.repro",
                        first_specification.file_name().unwrap().to_str().unwrap()
                    ))
                    .unwrap(),
                );
            }
        }

        // If the user explicitly specified that they don't want a reproduction
        // file, disable this feature.
        if dqcsim_opts.no_repro_out {
            config.reproduction_file.take();
        }

        Ok(config)
    }
}

fn format_error_msg(msg: &str) -> String {
    format!(
        "{} {}\n\nUSAGE:\n    {}\n\nFor more information try {}",
        Colour::Red.bold().paint("error:"),
        msg,
        include_str!("usage.txt").trim_end(),
        Colour::Green.normal().paint("--help")
    )
}

fn format_error<T>(e: impl Into<Error>) -> Result<T, Error> {
    Err(CommandLineError::Unknown(format_error_msg(&e.into().to_string())).into())
}

fn format_error_ctxt<T>(ctxt: &str, e: impl Into<Error>) -> Result<T, Error> {
    Err(CommandLineError::Unknown(format_error_msg(&format!(
        "{}: {}",
        ctxt,
        e.into().to_string()
    )))
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
        let c = CommandLineConfiguration {
            host_calls: vec![],
            host_stdout: true,
            dqcsim: SimulatorConfiguration::default().with_seed("test"),
            reproduction_file: None,
        };

        assert_eq!(format!("{:?}", c), "CommandLineConfiguration { host_calls: [], host_stdout: true, dqcsim: SimulatorConfiguration { seed: Seed { value: 14402189752926126668 }, stderr_level: Info, tee_files: [], log_callback: None, dqcsim_level: Trace, plugins: [], reproduction_path_style: Some(Keep) }, reproduction_file: None }");
    }

    #[test]
    fn help() {}
}
