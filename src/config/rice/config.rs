use crate::cli::InputSource;
use crate::config::app::Config;
use crate::config::rice::modified_file::ModifiedFile;
use crate::package_manager::dependency::Dependency;
use crate::system::io::{check_hash, cleanup_path, get_backup_path, get_temp_dir};
use crate::system::operating_system::OperatingSystem;
use crate::system::web::stream_file;
use color_eyre::eyre::{bail, eyre};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, info, trace, warn};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rice {
    pub name: String,
    pub description: String,
    pub version: String,
    pub authors: Vec<String>,
    pub operating_system: OperatingSystem,
    pub files: Vec<ModifiedFile>,
    pub dependencies: HashMap<OperatingSystem, Vec<Dependency>>,
}

impl Rice {
    pub async fn apply(config: &mut Config, source: &InputSource) -> color_eyre::Result<()> {
        info!("Applying rice configuration from {:?}", source);
        let rice = match source {
            InputSource::Path(path) => Self::load_from_file(&config.rice_folder, path).await,
            InputSource::Url(url) => Self::load_from_url(&config.rice_folder, url).await,
        }?;

        info!(
            "Rice: {} (v{}) by {:?}",
            rice.name, rice.version, rice.authors
        );
        debug!(description = %rice.description, "Rice details");

        let dependencies = OperatingSystem::get_dependencies_for_current_os(&rice.dependencies)?;
        debug!(
            count = dependencies.len(),
            "Found dependencies for current OS"
        );

        Dependency::install_multiple(config, dependencies).await?;

        info!("Applying configuration files");
        Self::apply_files(&rice.files).await?;

        info!("Rice applied successfully!");
        Ok(())
    }

    async fn apply_files(files: &Vec<ModifiedFile>) -> color_eyre::Result<()> {
        for (i, file) in files.iter().enumerate() {
            debug!(index = i, "Applying file");
            match file {
                ModifiedFile::Content {
                    path,
                    content: _,
                    sha256,
                } => {
                    info!("Writing content to {:?}", path);
                    let backup_path = get_backup_path(path).await;
                    trace!(?backup_path, "Creating backup");
                    if let Err(e) = tokio::fs::rename(path, &backup_path).await {
                        debug!(?path, ?e, "Failed to rename for backup (might not exist)");
                    }

                    tokio::fs::write(path, file.content().unwrap()).await?;

                    if let Some(expected) = sha256 {
                        debug!("Verifying SHA256 for {:?}", path);
                        check_hash(path, expected, path.display()).await?;
                    }
                }
                ModifiedFile::Remote { path, url, sha256 } => {
                    info!("Downloading remote file from {} to {:?}", url, path);
                    let backup_path = get_backup_path(path).await;
                    trace!(?backup_path, "Creating backup");
                    if let Err(e) = tokio::fs::rename(path, &backup_path).await {
                        debug!(?path, ?e, "Failed to rename for backup (might not exist)");
                    }

                    stream_file(url, path).await.map_err(|e| {
                        cleanup_path(path);
                        warn!(?url, ?e, "Download failed");
                        eyre!("Download failed for {}: {}", url, e)
                    })?;

                    if let Some(expected) = sha256 {
                        debug!("Verifying SHA256 for {:?}", path);
                        check_hash(path, expected, path.display()).await?;
                    }
                }
            }
        }

        Ok(())
    }

    async fn load_from_url(data_path: &Path, url: &Url) -> color_eyre::Result<Rice> {
        let tmp_file = get_temp_dir().join("tmp-rice-config.json");
        let _ = fs::remove_file(&tmp_file).await;
        if let Err(error) = stream_file(url, &tmp_file).await {
            let _ = fs::remove_file(&tmp_file).await;
            bail!("Could not stream file {}: {}", url, error)
        };
        Self::load_from_file(data_path, &tmp_file).await
    }

    async fn load_from_file(data_path: &Path, path: &PathBuf) -> color_eyre::Result<Rice> {
        let content = match fs::read_to_string(path).await {
            Ok(content) => content,
            Err(error) => {
                cleanup_path(path);
                bail!(
                    "Could not read content of file {}: {}",
                    path.display(),
                    error
                )
            }
        };
        let rice: Rice = match serde_json::from_str::<Rice>(&content) {
            Ok(rice) => rice,
            Err(error) => {
                cleanup_path(path);
                bail!(
                    "Could not parse content of file {}: {}",
                    path.display(),
                    error
                )
            }
        };

        let new_path = data_path.join(format!(
            "{} - {} - {}.json",
            &rice.name,
            rice.authors.first().unwrap_or(&"Unknown".into()),
            &rice.version
        ));
        match fs::copy(path, &new_path).await {
            Ok(_) => (),
            Err(error) => {
                cleanup_path(path);
                cleanup_path(&new_path);
                bail!(
                    "Could not copy file {} to new location {}: {}",
                    &path.display(),
                    &new_path.display(),
                    error
                )
            }
        };
        Ok(rice)
    }
}
