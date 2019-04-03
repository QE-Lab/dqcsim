use crate::{
    common::{
        error::{inv_arg, oe_err, Result},
        log::{tee_file::TeeFile, Loglevel, LoglevelFilter},
        types::ArbCmd,
    },
    host::configuration::{
        env_mod::EnvMod, stream_capture_mode::StreamCaptureMode, timeout::Timeout, PluginType,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    env::{current_exe, split_paths, var_os},
    ffi::OsString,
    path::PathBuf,
};

/// Plugin specification, consisting of the executable filename for the plugin
/// and an optional script filename for it to execute for when the executable
/// is an interpreter.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PluginProcessSpecification {
    /// The original, sugared specification, if any.
    #[serde(skip)]
    pub sugared: Option<PathBuf>,

    /// The executable filename of the plugin.
    pub executable: PathBuf,

    /// If specified, the executable is expected to be an interpreter, which is
    /// to execute the specified script file. If not specified, the executable
    /// is a native plugin.
    pub script: Option<PathBuf>,

    /// Plugin type.
    pub typ: PluginType,
}

impl PluginProcessSpecification {
    /// Constructs a new plugin specification.
    pub fn new<T>(
        executable: impl Into<PathBuf>,
        script: Option<T>,
        typ: impl Into<PluginType>,
    ) -> PluginProcessSpecification
    where
        T: Into<PathBuf>,
    {
        PluginProcessSpecification {
            sugared: None,
            executable: executable.into(),
            script: script.map(std::convert::Into::into),
            typ: typ.into(),
        }
    }

    /// Constructs a plugin specification from a "sugared" specification.
    ///
    /// The specification can take the following forms:
    ///
    /// - a valid path to a plugin executable with no file extension;
    /// - the basename of a plugin executable with no file extension with
    ///   implicit "dqcsfe"/"dqcsop"/"dqcsbe" prefix, searched for in A) the
    ///   working directory, B) the binary directory, and C) the system $PATH;
    /// - a valid path to a script file with a file extension. In this case,
    ///   the above rule is run for a plugin named by the file extension of the
    ///   script file. For instance, if "test.py" is specified for the frontend,
    ///   this will look for an executable named "dqcsfepy".
    ///
    /// Failure to find the plugin executable or script file results in an
    /// error being returned.
    pub fn from_sugar(
        specification: impl Into<PathBuf>,
        typ: PluginType,
    ) -> Result<PluginProcessSpecification> {
        // Generate the default specification. This default assumes that the
        // specification is a valid path to an executable. We'll fix the
        // structure later if this assumption turns out to be incorrect.
        let specification = specification.into();
        let mut specification = PluginProcessSpecification {
            sugared: Some(specification.clone()),
            executable: specification,
            script: None,
            typ,
        };

        // Handle the simple cases, where the specification is a path to an
        // existing file.
        if specification.executable.exists() {
            if specification.executable.extension().is_some() {
                // The file that we assumed to be the executable is actually a
                // script file. Set the executable to just the file extension;
                // we desugar that later.
                specification.script = Some(specification.executable);
                specification.executable = specification
                    .script
                    .as_ref()
                    .unwrap()
                    .extension()
                    .unwrap()
                    .into();
            } else {
                // Our assumptions appear to be correct.
                return Ok(specification);
            }
        } else {
            // The executable does not exist. If it doesn't contain slashes,
            // we interpret it as a sugared plugin name. If it does contain
            // slashes, the user probably tried to give us an existing path
            // but made a mistake, so we return an error.
            if specification
                .executable
                .as_os_str()
                .to_string_lossy()
                .contains('/')
            {
                return inv_arg(format!(
                    "the plugin specification '{}' appears to be a path, \
                     but the referenced file does not exist",
                    &specification.executable.to_string_lossy()
                ));
            }
        }

        // The executable does not exist (or is just a file extension and
        // should always be treated as sugar). Before we look for the plugin
        // elsewhere, add the appropriate prefix.
        let mut prefix: OsString = match typ {
            PluginType::Frontend => "dqcsfe",
            PluginType::Operator => "dqcsop",
            PluginType::Backend => "dqcsbe",
        }
        .into();
        prefix.push(specification.executable.as_os_str());
        specification.executable = prefix.into();

        // If the executable exists now, i.e. there is a file with the right
        // name in the working directory, we're done.
        if specification.executable.exists() {
            return Ok(specification);
        }

        // Look for the file in the directory where DQCsim resides.
        if let Ok(dqcsim_dir) = current_exe() {
            let mut exec = dqcsim_dir
                .parent()
                .ok_or_else(oe_err("Could not determine path to DQCsim binary."))?
                .to_path_buf();
            exec.push(&specification.executable);
            if exec.exists() {
                specification.executable = exec;
                return Ok(specification);
            }
        }

        // Okay, still not found. Try the system path then.
        if let Some(sys_path) = var_os("PATH") {
            for base in split_paths(&sys_path) {
                let mut exec = base.clone();
                exec.push(&specification.executable);
                if exec.exists() {
                    specification.executable = exec;
                    return Ok(specification);
                }
            }
        }

        inv_arg(format!(
            "could not find plugin executable '{}', needed for plugin \
             specification '{}'",
            specification.executable.to_string_lossy(),
            specification.sugared.unwrap().to_string_lossy(),
        ))
    }
}

/// Structure describing the functional configuration of a plugin, i.e. the
/// parameters that affect a plugin's behavior.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PluginProcessFunctionalConfiguration {
    /// ArbCmd objects passed to the plugin initialization RPC.
    pub init: Vec<ArbCmd>,

    /// Environment variable overrides for the plugin process.
    pub env: Vec<EnvMod>,

    /// The working directory for the plugin process.
    pub work: PathBuf,
}

impl Default for PluginProcessFunctionalConfiguration {
    fn default() -> PluginProcessFunctionalConfiguration {
        PluginProcessFunctionalConfiguration {
            init: vec![],
            env: vec![],
            work: ".".into(),
        }
    }
}

/// Structure describing the NONfunctional configuration of a plugin, i.e. the
/// parameters that only affect how the plugin represents its output.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PluginProcessNonfunctionalConfiguration {
    /// Specifies the verbosity of the messages sent to DQCsim.
    pub verbosity: LoglevelFilter,

    /// Specifies the tee files for this plugin.
    pub tee_files: Vec<TeeFile>,

    /// Specifies how the stdout stream of the plugin should be connected.
    pub stdout_mode: StreamCaptureMode,

    /// Specifies how the stderr stream of the plugin should be connected.
    pub stderr_mode: StreamCaptureMode,

    /// Specifies the timeout for connecting to the plugin after it has been
    /// spawned.
    pub accept_timeout: Timeout,

    /// Specifies the timeout duration to wait for the plugin to shutdown after
    /// sending the abort request.
    pub shutdown_timeout: Timeout,
}

impl Default for PluginProcessNonfunctionalConfiguration {
    fn default() -> PluginProcessNonfunctionalConfiguration {
        PluginProcessNonfunctionalConfiguration {
            verbosity: LoglevelFilter::Info,
            tee_files: vec![],
            stdout_mode: StreamCaptureMode::Capture(Loglevel::Info),
            stderr_mode: StreamCaptureMode::Capture(Loglevel::Info),
            accept_timeout: Timeout::from_seconds(5),
            shutdown_timeout: Timeout::from_seconds(5),
        }
    }
}

/// Represents the complete configuration for a plugin.
///
/// In combination with some modifiers and defaults set by DQCsim itself, this
/// contains everything needed to construct a plugin.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PluginProcessConfiguration {
    /// Name of the plugin, used to refer to the plugin by the log system.
    pub name: String,

    /// Plugin specification, i.e. the plugin executable and optionally the
    /// script it should execute.
    pub specification: PluginProcessSpecification,

    /// The functional configuration of the plugin, i.e. the options
    /// configuring how the plugin behaves (besides the specification).
    pub functional: PluginProcessFunctionalConfiguration,

    /// The nonfunctional configuration of the plugin, i.e. any options that
    /// do not affect how the plugin behaves functionally, but only affect its
    /// output representation.
    pub nonfunctional: PluginProcessNonfunctionalConfiguration,
}

impl PluginProcessConfiguration {
    /// Creates a new plugin configuration.
    ///
    /// The default values are inserted for the configuration options.
    pub fn new(
        name: impl Into<String>,
        specification: PluginProcessSpecification,
    ) -> PluginProcessConfiguration {
        PluginProcessConfiguration {
            name: name.into(),
            specification,
            functional: PluginProcessFunctionalConfiguration::default(),
            nonfunctional: PluginProcessNonfunctionalConfiguration::default(),
        }
    }
}
