use config::{Config, ConfigError, Environment, File, FileFormat, Source, Value};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug, SmartDefault)]
#[serde(default)]
pub struct Conf {
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
