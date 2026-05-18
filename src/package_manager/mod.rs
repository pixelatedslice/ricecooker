pub mod dependency;
pub mod format;
pub mod setting;

use crate::config::app::Config;
use crate::package_manager::format::PackageFormat;
use crate::source::Source;
use crate::system::cmd::run_command_in_shell;
use crate::system::io::get_temp_dir;
use crate::system::operating_system::OperatingSystem;
use crate::system::web::stream_file;
use aho_corasick::AhoCorasick;
use color_eyre::eyre::bail;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use tokio::fs;
use tracing::{debug, info, trace, warn};
use url::Url;
use which::which;

pub static INSTALL_COMMAND_AC: LazyLock<AhoCorasick> =
    LazyLock::new(|| AhoCorasick::new(["{pkgs}"]).unwrap());
pub static VERSIONED_PACKAGE_FORMAT_AC: LazyLock<AhoCorasick> =
    LazyLock::new(|| AhoCorasick::new(["{pkg}", "{ver}"]).unwrap());
pub static UNVERSIONED_PACKAGE_FORMAT_AC: LazyLock<AhoCorasick> =
    LazyLock::new(|| AhoCorasick::new(["{pkg}"]).unwrap());

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PackageManager {
    #[serde(flatten)]
    pub operating_systems: Vec<OperatingSystem>,
    pub binary_name: String,
    pub update_command: String,
    pub install_command: String,
    pub package_format: PackageFormat,
}

impl PackageManager {
    pub async fn load(config: &Config, source: Source) -> color_eyre::Result<PackageManager> {
        debug!(?source, "Loading package manager");
        match source {
            Source::Local { path } => {
                PackageManager::load_from_file(&config.package_manager_folder, &path).await
            }
            Source::Remote { url } => {
                PackageManager::load_from_url(&config.package_manager_folder, &url).await
            }
        }
    }

    async fn load_from_url(data_path: &Path, url: &Url) -> color_eyre::Result<PackageManager> {
        debug!(%url, "Loading package manager from URL");
        let tmp_file = get_temp_dir().join("tmp-pm-config.json");
        let _ = fs::remove_file(&tmp_file).await;
        stream_file(url, &tmp_file).await?;
        PackageManager::load_from_file(data_path, &tmp_file).await
    }

    async fn load_from_file(
        data_path: &Path,
        path: &PathBuf,
    ) -> color_eyre::Result<PackageManager> {
        debug!(?path, "Loading package manager from file");
        let content = fs::read_to_string(path).await?;
        let pm: PackageManager = serde_json::from_str(&content)?;

        let dest = data_path.join(&pm.binary_name);
        trace!(?dest, "Copying package manager config to data path");
        fs::copy(path, dest).await?;

        info!("Loaded package manager: {}", pm.binary_name);
        Ok(pm)
    }

    pub async fn update(&self) -> color_eyre::Result<()> {
        info!("Updating package manager: {}", self.binary_name);
        debug!(command = %self.update_command, "Running update command");
        run_command_in_shell(&self.update_command)
    }

    pub async fn install(&self, packages: &[(&String, &Option<String>)]) -> color_eyre::Result<()> {
        info!(
            "Installing {} packages using {}",
            packages.len(),
            self.binary_name
        );
        let packages_str = packages
            .iter()
            .map(|(name, version)| {
                let formatted = match version {
                    Some(version) => VERSIONED_PACKAGE_FORMAT_AC
                        .replace_all(&self.package_format.versioned, &[name, version].clone()),
                    None => UNVERSIONED_PACKAGE_FORMAT_AC
                        .replace_all(&self.package_format.unversioned, &[name].clone()),
                };
                trace!(?name, ?version, ?formatted, "Formatted package");
                formatted
            })
            .collect::<Vec<_>>()
            .join(" ");

        let command = INSTALL_COMMAND_AC.replace_all(&self.install_command, &[packages_str]);
        debug!(%command, "Running install command");
        run_command_in_shell(command)
    }

    pub async fn find_installed_manager(
        package_manager_dir: &PathBuf,
    ) -> color_eyre::Result<(PackageManager, PathBuf)> {
        debug!(
            ?package_manager_dir,
            "Searching for installed package manager"
        );
        let mut entries = fs::read_dir(package_manager_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            trace!(?path, "Checking file for package manager config");
            let content = fs::read_to_string(&path).await?;
            if let Ok(package_manager) = serde_json::from_str::<PackageManager>(&content) {
                if which(&package_manager.binary_name).is_ok() {
                    info!(
                        "Found installed package manager: {} at {:?}",
                        package_manager.binary_name, path
                    );
                    return Ok((package_manager, path));
                } else {
                    debug!(binary = %package_manager.binary_name, "Binary not found in PATH for config at {:?}", path);
                }
            }
        }

        warn!(
            "No installed package manager found in {:?}",
            package_manager_dir
        );
        bail!(
            "No package manager found in {}",
            package_manager_dir.display()
        )
    }
}
