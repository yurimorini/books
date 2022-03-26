use super::Args;
use serde::{Deserialize, Serialize};

/// App config structure
///
/// - base_url: Url for the book resolution service
/// - api_key: API key for the book resolution service
/// - output: Output path for the library store
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub base_url: String,
    pub api_key: String,
    pub output: String,
}

/////////////////////////////////////////////////////////////////////////////
// Type implementation
/////////////////////////////////////////////////////////////////////////////

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            base_url: "".into(),
            api_key: "".into(),
            output: "library.json".into(),
        }
    }
}

impl Config {
    /// Set default values from Args.
    ///
    pub fn set_default(&mut self, arguments: &Args) -> &Config {
        self.output = arguments.output_file.clone();
        self
    }

    /// Base url and api key are needed by the client
    /// Not having them should flag the config as invalid
    ///
    pub fn is_valid(&self) -> bool {
        self.base_url != "" && self.api_key != ""
    }
}
