use super::Args;
use crate::config::Config;
use home_dir::HomeDirExt;
use std::fs::OpenOptions;
use std::fs::{create_dir_all, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use thiserror::Error;

/// Translate the input option to a config file.
///
pub struct ConfigBuilder;

/// Input error to wrap all errors
///
#[derive(Error, Debug)]
pub enum BuildError {
    #[error("Impossible to create dir")]
    ConfigDirCannotBeCreated,
    #[error("Error defining base dir")]
    ConfigDirNotDefined,
    #[error("Impossible to write config file")]
    ConfigFileIoError,
    #[error("Invalid configuration provided")]
    InvalidConfig,
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////

impl ConfigBuilder {
    /// Load the config file from location taken from CLI args.
    /// If the file does not exists, create it in a default folder and use it
    ///
    pub fn from_cli_args(cli: &Args) -> Result<Config, BuildError> {
        let config_file = ConfigBuilder::create(&cli.config_file, None)?;

        let mut config = ConfigBuilder::load(&config_file);
        config.set_default(&cli);

        if !config.is_valid() {
            return Err(BuildError::InvalidConfig);
        }

        Ok(config)
    }

    /// Load the configuration from disk.
    ///
    /// It search always a config.json file.
    ///
    fn load(config_file: &str) -> Config {
        read_to_string(config_file)
            .map(|data| serde_json::from_str(&data))
            .unwrap_or_else(|_| Ok(Config::default()))
            .unwrap()
    }

    /// Create the config file.
    ///
    fn create(path: &str, config: Option<&Config>) -> Result<String, BuildError> {
        let expanded = PathBuf::from(path).expand_home().ok();

        let exists = match expanded.clone() {
            Some(path) => path.exists(),
            None => false,
        };

        if !exists {
            let base = expanded
                .clone()
                .and_then(|p| p.parent().map(|p| p.to_owned()));

            let result = base
                .map(|path| {
                    create_dir_all(path).map_err(|_| BuildError::ConfigDirCannotBeCreated)
                })
                .unwrap_or_else(|| Err(BuildError::ConfigDirNotDefined));

            if let Err(x) = result {
                return Err(x);
            };
        }

        let path = expanded.unwrap();
        let expanded_path = path.to_string_lossy().to_string();
        let default = &Config::default();

        if exists {
            return Ok(expanded_path);
        }

        let config = match config {
            Some(config) => config,
            None => default,
        };

        let wrote = serde_json::to_string_pretty(config).map(|data| {
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .and_then(|mut f| f.write_all(data.as_bytes()))
        });

        if wrote.is_ok() {
            Ok(expanded_path)
        } else {
            Err(BuildError::ConfigFileIoError)
        }
    }
}
