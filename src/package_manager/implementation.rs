use crate::config::app::config::Config;
use crate::package_manager::dependency::Dependency;
use crate::package_manager::{
    PackageManager, PackageManagerSource, INSTALL_COMMAND_AC, UNVERSIONED_PACKAGE_FORMAT_AC,
    VERSIONED_PACKAGE_FORMAT_AC,
};
use crate::system::cmd::run_command_in_shell;
use crate::system::directories::get_temp_dir;
use crate::system::web::stream_file;
use color_eyre::eyre::eyre;
use std::path::{Path, PathBuf};
use tokio::fs;
use url::Url;
use which::which;

impl PackageManager {
    pub async fn load(
        config: &Config,
        source: PackageManagerSource,
    ) -> color_eyre::Result<PackageManager> {
        match source {
            PackageManagerSource::Local { path } => {
                PackageManager::load_from_file(&config.data_path.join("package_managers"), &path)
                    .await
            }
            PackageManagerSource::Remote { url } => {
                PackageManager::load_from_url(&config.data_path.join("package_managers"), &url)
                    .await
            }
        }
    }

    async fn load_from_url(data_path: &Path, url: &Url) -> color_eyre::Result<PackageManager> {
        let tmp_file = get_temp_dir().join("tmp-pm-config.json");
        fs::remove_file(&tmp_file).await?;
        stream_file(url, &tmp_file).await?;
        PackageManager::load_from_file(data_path, &tmp_file).await
    }

    async fn load_from_file(
        data_path: &Path,
        path: &PathBuf,
    ) -> color_eyre::Result<PackageManager> {
        let content = fs::read_to_string(path).await?;
        let pm: PackageManager = serde_json::from_str(&content)?;

        fs::copy(path, data_path.join(&pm.binary_name)).await?;

        Ok(pm)
    }

    pub async fn update(&self) -> color_eyre::Result<()> {
        run_command_in_shell(&self.update_command)
    }

    pub async fn install(&self, packages: &[Dependency]) -> color_eyre::Result<()> {
        let packages = packages
            .iter()
            .filter(|dependency| matches!(dependency, Dependency::PackageManager { .. }))
            .map(|dependency| match dependency {
                Dependency::PackageManager { name, version } => (name, version),
                _ => unreachable!(),
            })
            .map(|(name, version)| match version {
                Some(version) => VERSIONED_PACKAGE_FORMAT_AC
                    .replace_all(&self.package_format.versioned, &[name, version]),
                None => UNVERSIONED_PACKAGE_FORMAT_AC
                    .replace_all(&self.package_format.unversioned, &[name]),
            })
            .collect::<Vec<_>>()
            .join(" ");

        let command = INSTALL_COMMAND_AC.replace_all(&self.install_command, &[packages]);
        run_command_in_shell(command)
    }

    pub async fn find_installed_manager(
        package_manager_dir: &PathBuf,
    ) -> color_eyre::Result<(PackageManager, PathBuf)> {
        let mut entries = fs::read_dir(package_manager_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let content = fs::read_to_string(&path).await?;
            if let Ok(package_manager) = serde_json::from_str::<PackageManager>(&content) {
                if which(&package_manager.binary_name).is_ok() {
                    return Ok((package_manager, path));
                }
            }
        }

        Err(eyre!(
            "No package manager found in {}",
            package_manager_dir.display()
        ))
    }
}
