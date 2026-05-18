use color_eyre::eyre::bail;
use sha2::{Digest, Sha256};
use std::env;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncReadExt;
use tracing::{debug, trace, warn};

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

pub fn cleanup_path<P: AsRef<Path>>(path: P) {
    let owned_path = path.as_ref().to_path_buf();
    debug!(?owned_path, "Cleaning up path");

    tokio::task::spawn_blocking(async move || {
        if owned_path.is_file() {
            if let Err(e) = std::fs::remove_file(&owned_path) {
                warn!(?owned_path, ?e, "Failed to remove file during cleanup");
            }
        } else if owned_path.is_dir() {
            if let Err(e) = std::fs::remove_dir_all(&owned_path) {
                warn!(?owned_path, ?e, "Failed to remove directory during cleanup");
            }
        }
    });
}

pub async fn get_backup_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    trace!(?path, "Calculating backup path");

    let Some(parent) = path.parent() else {
        return path.with_added_extension("bak0");
    };

    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return path.with_added_extension("bak0");
    };

    let backup_prefix = format!("{file_name}.bak");

    let number = std::fs::read_dir(parent)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter_map(|name| name.strip_prefix(&backup_prefix)?.parse::<i32>().ok())
        .max()
        .map_or(0, |number| number + 1);

    let backup_path = path.with_added_extension(format!("bak{number}"));
    trace!(?backup_path, "Calculated backup path");
    backup_path
}

pub async fn hash_file<P: AsRef<Path>>(path: P) -> color_eyre::Result<String> {
    let path = path.as_ref();
    trace!(?path, "Hashing file");
    let mut file = fs::File::open(path).await?;
    let mut hasher = Sha256::new();

    let mut buffer = [0; 131072]; // 128 * 1024

    loop {
        let count = file.read(&mut buffer).await?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let hash = hex::encode(hasher.finalize());
    trace!(?path, %hash, "Hash calculated");
    Ok(hash)
}

pub async fn check_hash<P: AsRef<Path>, D: Display>(
    path: P,
    expected: &String,
    for_display: D,
) -> color_eyre::Result<()> {
    let path = path.as_ref();
    debug!(?path, %expected, "Checking file hash");

    let actual = hash_file(path).await?;
    if &actual != expected {
        warn!(?path, %expected, %actual, "Hash mismatch!");
        cleanup_path(path);

        bail!(
            "SHA256 checksum mismatch for {}: expected {}, got {}",
            for_display,
            expected,
            actual
        );
    }

    debug!(?path, "Hash verification successful");
    Ok(())
}
