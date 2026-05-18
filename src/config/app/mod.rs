use crate::package_manager::setting::PackageManagerSetting;
use crate::package_manager::PackageManager;
use crate::source::Source;
use crate::system::io::{get_config_dir, get_data_path, get_temp_dir};
use color_eyre::eyre::{bail, WrapErr};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tracing::{debug, info, trace};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(skip)]
    pub config_dir: PathBuf,

    #[serde(skip)]
    pub temp_dir: PathBuf,

    #[serde(skip)]
    pub data_path: PathBuf,

    #[serde(skip)]
    pub rice_folder: PathBuf,

    #[serde(skip)]
    pub package_manager_folder: PathBuf,

    #[serde(
        serialize_with = "PackageManagerSetting::serialize",
        deserialize_with = "PackageManagerSetting::deserialize"
    )]
    package_manager_setting: PackageManagerSetting,
}

impl Config {
    async fn new(
        config_dir: PathBuf,
        temp_dir: PathBuf,
        data_path: PathBuf,
        rice_folder: PathBuf,
        package_manager_folder: PathBuf,
    ) -> color_eyre::Result<Config> {
        let config = Config {
            config_dir,
            temp_dir,
            data_path,
            rice_folder,
            package_manager_folder,
            package_manager_setting: PackageManagerSetting::None,
        };

        config.save().await?;

        Ok(config)
    }

    pub async fn load() -> color_eyre::Result<Config> {
        info!("Loading application configuration");
        let config_dir = get_config_dir();
        let data_path = get_data_path();
        let package_manager_folder = get_data_path().join("package_manager");
        let rice_folder = get_data_path().join("rice");
        let temp_dir = get_temp_dir();

        trace!(
            ?config_dir,
            ?data_path,
            ?package_manager_folder,
            ?rice_folder,
            ?temp_dir,
            "Configuration paths"
        );

        let (res1, res2, res3, res4, res5) = tokio::join!(
            tokio::fs::create_dir_all(config_dir.clone()),
            tokio::fs::create_dir_all(data_path.clone()),
            tokio::fs::create_dir_all(package_manager_folder.clone()),
            tokio::fs::create_dir_all(rice_folder.clone()),
            tokio::fs::create_dir_all(temp_dir.clone()),
        );
        res1?;
        res2?;
        res3?;
        res4?;
        res5?;

        let config_path = config_dir.join("config.json");

        if !fs::try_exists(&config_path).await? {
            debug!(
                "Config file not found, creating new one at {:?}",
                config_path
            );
            return Self::new(
                config_dir,
                temp_dir,
                data_path,
                rice_folder,
                package_manager_folder,
            )
            .await;
        };

        debug!("Reading config file from {:?}", config_path);
        let content = fs::read_to_string(&config_path).await?;
        let mut config = serde_json::from_str::<Config>(&content)?;

        match &config.package_manager_setting {
            PackageManagerSetting::None => {
                debug!("No package manager setting found, searching for installed manager");
                let (current, path) =
                    PackageManager::find_installed_manager(&package_manager_folder).await?;
                config.package_manager_setting = PackageManagerSetting::Current {
                    preferred: path,
                    current,
                };
            }
            PackageManagerSetting::Preferred(preferred) => {
                debug!(?preferred, "Loading preferred package manager");
                let package_manager = PackageManager::load(
                    &config,
                    Source::Local {
                        path: preferred.clone(),
                    },
                )
                .await?;

                config.package_manager_setting = PackageManagerSetting::Current {
                    preferred: preferred.clone(),
                    current: package_manager,
                };
            }
            PackageManagerSetting::Current { .. } => {
                trace!("Using currently loaded package manager");
            }
        }

        Ok(config)
    }

    pub async fn save(&self) -> color_eyre::Result<()> {
        let config_path = self.config_dir.join("config.json");
        debug!("Saving configuration to {:?}", config_path);
        fs::write(config_path, serde_json::to_string_pretty(self)?)
            .await
            .wrap_err("Failed to save configuration file")
    }

    pub async fn set_package_manager(
        &mut self,
        package_manager: PackageManager,
    ) -> color_eyre::Result<()> {
        self.package_manager_setting = PackageManagerSetting::Current {
            preferred: self
                .package_manager_folder
                .join(&package_manager.binary_name),
            current: package_manager,
        };

        Ok(())
    }

    pub async fn get_package_manager(&mut self) -> color_eyre::Result<&PackageManager> {
        match &self.package_manager_setting {
            PackageManagerSetting::None => bail!("No package manager specified."),
            PackageManagerSetting::Preferred(preferred) => {
                let package_manager = PackageManager::load(
                    self,
                    Source::Local {
                        path: preferred.clone(),
                    },
                )
                .await?;

                self.package_manager_setting = PackageManagerSetting::Current {
                    preferred: preferred.clone(),
                    current: package_manager.clone(),
                };
            }
            PackageManagerSetting::Current {
                preferred: _,
                current: _,
            } => (),
        }

        if let PackageManagerSetting::Current { current, .. } = &self.package_manager_setting {
            Ok(current)
        } else {
            bail!("Error while getting package manager.")
        }
    }
}
