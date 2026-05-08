use std::env;
use std::path::PathBuf;

mod app;
mod config;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let config = config::app_config::AppConfig::load()?;

    Ok(())
}

pub fn get_config_dir() -> PathBuf {
    if let Ok(path) = env::var("XDG_CONFIG_HOME") {
        return [path.as_str(), "ricecooker"].iter().collect();
    }

    let home: String = env::var("HOME").expect("HOME environment variable not set");
    [home.as_str(), ".config", "ricecooker"].iter().collect()
}

pub fn get_data_path() -> PathBuf {
    if let Ok(path) = env::var("XDG_DATA_HOME") {
        return [path.as_str(), "ricecooker"].iter().collect();
    }

    let home: String = env::var("HOME").expect("HOME environment variable not set");
    [home.as_str(), ".local", "share", "ricecooker"]
        .iter()
        .collect()
}
