use dqcsim::{configuration::*, log::tee_file::TeeFile, log::*};
use failure::{Error, Fail};

/// Structure containing the NONfunctional options for a plugin, i.e. the
/// options that only affect how the plugin represents its output.
///
/// This differs from `PluginNonfunctionalConfiguration` in that unspecified values
/// have not yet been replaced with their defaults. This allows the structure
/// to be built incrementally (see `apply()`).
#[derive(Debug)]
pub struct PluginNonfunctionalOpts {
    /// Specifies the verbosity of the messages sent to DQCsim. If this is
    /// `None`, the value of DQCsim's `--plugin_level` option should be used.
    pub verbosity: Option<LoglevelFilter>,

    /// Specifies the tee files for this plugin.
    pub tee_files: Vec<TeeFile>,

    /// Specifies how the stdout stream of the plugin should be connected.
    /// `None` implies default.
    pub stdout_mode: Option<StreamCaptureMode>,

    /// Specifies how the stderr stream of the plugin should be connected.
    /// `None` implies default.
    pub stderr_mode: Option<StreamCaptureMode>,

    /// Specifies the timeout for connecting to the plugin after it has been
    /// spawned.
    pub accept_timeout: Option<Timeout>,

    /// Specifies the timeout for connecting to the plugin after it has been
    /// spawned.
    pub shutdown_timeout: Option<Timeout>,
}

impl PluginNonfunctionalOpts {
    /// Applies these plugin nonfunctional configuration modifications to a
    /// nonfunctional plugin configuration.
    ///
    /// An error is returned if the referenced plugin cannot be found in the
    /// vector, otherwise `Ok(())` is returned.
    pub fn apply(&self, to: &mut PluginNonfunctionalConfiguration) {
        if let Some(verbosity) = self.verbosity {
            to.verbosity = verbosity;
        }
        to.tee_files.extend(self.tee_files.iter().cloned());
        if let Some(stdout_mode) = &self.stdout_mode {
            to.stdout_mode = stdout_mode.clone();
        }
        if let Some(stderr_mode) = &self.stderr_mode {
            to.stderr_mode = stderr_mode.clone();
        }
        if let Some(accept_timeout) = &self.accept_timeout {
            to.accept_timeout = accept_timeout.clone();
        }
        if let Some(shutdown_timeout) = &self.shutdown_timeout {
            to.shutdown_timeout = shutdown_timeout.clone();
        }
    }

    /// Converts this structure to a PluginNonfunctionalConfiguration structure by
    /// replacing unset values with their defaults.
    pub fn into_config(
        self,
        default_verbosity: LoglevelFilter,
    ) -> PluginNonfunctionalConfiguration {
        PluginNonfunctionalConfiguration {
            verbosity: self.verbosity.unwrap_or(default_verbosity),
            tee_files: self.tee_files,
            stdout_mode: self
                .stdout_mode
                .unwrap_or(StreamCaptureMode::Capture(Loglevel::Info)),
            stderr_mode: self
                .stderr_mode
                .unwrap_or(StreamCaptureMode::Capture(Loglevel::Info)),
            accept_timeout: self.accept_timeout.unwrap_or(Timeout::from_seconds(5)),
            shutdown_timeout: self.shutdown_timeout.unwrap_or(Timeout::from_seconds(5)),
        }
    }
}

impl Default for PluginNonfunctionalOpts {
    fn default() -> Self {
        PluginNonfunctionalOpts {
            verbosity: None,
            tee_files: vec![],
            stdout_mode: None,
            stderr_mode: None,
            accept_timeout: None,
            shutdown_timeout: None,
        }
    }
}

/// Represents the definition of a plugin.
///
/// In combination with some modifiers and defaults set by DQCsim itself, this
/// contains everything needed to construct a plugin.
#[derive(Debug)]
pub struct PluginDefinition {
    /// Name of the plugin, used to refer to the plugin by the log system and
    /// by `PluginModification`s.
    pub name: String,

    /// Plugin specification, i.e. the plugin executable and optionally the
    /// script it should execute.
    pub specification: PluginSpecification,

    /// The functional configuration of the plugin, i.e. the options
    /// configuring how the plugin behaves (besides the specification).
    pub functional: PluginFunctionalConfiguration,

    /// The nonfunctional configuration of the plugin, i.e. any options that
    /// do not affect how the plugin behaves functionally, but only affect its
    /// output representation.
    pub nonfunctional: PluginNonfunctionalOpts,
}

impl PluginDefinition {
    /// Converts this structure to a PluginConfiguration structure by
    /// replacing unset values with their defaults.
    pub fn into_config(self, default_verbosity: LoglevelFilter) -> PluginConfiguration {
        PluginConfiguration {
            name: self.name,
            specification: self.specification,
            functional: self.functional,
            nonfunctional: self.nonfunctional.into_config(default_verbosity),
        }
    }
}

/// Error structure used for reporting PluginModification errors.
#[derive(Debug, Fail, PartialEq)]
pub enum PluginModificationError {
    #[fail(display = "{}", 0)]
    NotFound(String),
}

/// Represents the modification of a previously defined plugin.
///
/// This allows the nonfunctional configuration of the plugin to be modified.
/// This is particularly important when DQCsim is running in
/// --reproduce-exactly mode; it is of marginal use to reproduce a run exactly
/// if you're not looking to get additional information from it by changing
/// loglevels and such.
#[derive(Debug)]
pub struct PluginModification {
    /// Name of the referenced plugin.
    pub name: String,

    /// Overrides for the nonfunctional plugin configuration.
    pub nonfunctional: PluginNonfunctionalOpts,
}

impl PluginModification {
    /// Applies this plugin modification to a plugin definition vector.
    ///
    /// An error is returned if the referenced plugin cannot be found in the
    /// vector, otherwise `Ok(())` is returned.
    pub fn apply(self, to: &mut Vec<PluginConfiguration>) -> Result<(), Error> {
        for plugin_config in &mut to.iter_mut() {
            if plugin_config.name == self.name {
                self.nonfunctional.apply(&mut plugin_config.nonfunctional);
                return Ok(());
            }
        }
        Err(PluginModificationError::NotFound(format!(
            "There is no plugin named {}. The available plugins are {}.",
            self.name,
            enum_variants::friendly_enumerate(to.iter().map(|x| &x.name[..]), Some("or"))
        ))
        .into())
    }
}
