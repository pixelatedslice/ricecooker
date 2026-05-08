use crate::config::rice_config::RiceConfig;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    #[serde(skip_serializing, skip_deserializing)]
    config_path: PathBuf,
    pub data_path: PathBuf,
    #[serde(skip_serializing, default)]
    pub rice_configs: Vec<RiceConfig>,
}

impl AppConfig {
    fn new(config_path: PathBuf, data_path: PathBuf) -> Result<Self, Error> {
        let new = Self {
            config_path,
            data_path,
            rice_configs: vec![],
        };
        new.save()?;

        Ok(new)
    }

    pub fn load() -> Result<Self, Error> {
        let config_path = crate::get_config_dir().join("config.json");

        if !config_path.exists() {
            return Self::new(config_path, crate::get_data_path());
        }

        let content = fs::read_to_string(&config_path)?;

        let mut config: Self = serde_json::from_str(&content)?;
        config.config_path = config_path;

        let entries = fs::read_dir(&config.data_path)?;
        let paths: Vec<PathBuf> = entries
            .filter_map(|entry| {
                let entry = entry.ok()?; // Handle IO errors
                if entry.path().is_file() {
                    Some(entry.path())
                } else {
                    None
                }
            })
            .collect();

        for path in paths {
            let rice_config = RiceConfig::from(fs::read_to_string(path)?);
            if (rice_config.is_err()) {
                continue;
            }
            config.rice_configs.push(rice_config?);
        }

        Ok(config)
    }

    pub fn save(&self) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&self.config_path, json)?;
        Ok(())
    }
}
