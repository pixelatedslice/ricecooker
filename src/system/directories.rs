use std::env;
use std::path::PathBuf;

pub fn get_home_env() -> String {
    env::var("HOME").expect("HOME environment variable not set")
}

pub fn get_config_dir() -> PathBuf {
    if let Ok(path) = env::var("XDG_CONFIG_HOME") {
        return [path.as_str(), "ricecooker"].iter().collect();
    }

    [get_home_env().as_str(), ".config", "ricecooker"]
        .iter()
        .collect()
}

pub fn get_data_path() -> PathBuf {
    if let Ok(path) = env::var("XDG_DATA_HOME") {
        return [path.as_str(), "ricecooker"].iter().collect();
    }

    [get_home_env().as_str(), ".local", "share", "ricecooker"]
        .iter()
        .collect()
}

pub fn get_temp_dir() -> PathBuf {
    env::temp_dir().join("ricecooker")
}
