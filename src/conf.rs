//! App config.

use config::{Config, ConfigError, Environment, File, FileFormat, Source, Value};
use serde::{Deserialize, Serialize};

/// Config struct
#[derive(Deserialize, Clone, Debug, SmartDefault)]
#[serde(default)]
pub struct Conf {
    /// Path to database where Plutonio will store data.
    /// If the Plutoni does not find a database along this path,
    /// then it will create it itself.
    #[default(".plutonio.db".to_string())]
    pub database_path: String,
}

impl Conf {
    pub fn parse() -> Result<Self, ConfigError> {
        let mut cfg = Config::new();
        cfg.merge(File::with_name("config.toml")).ok();

        cfg.try_into()
    }
}
