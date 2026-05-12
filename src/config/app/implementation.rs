use crate::config::app::config::{Config, PackageManagerSetting};
use crate::package_manager::{PackageManager, PackageManagerSource};
use crate::system::directories::{get_config_dir, get_data_path, get_temp_dir};
use color_eyre::eyre::WrapErr;
use std::path::PathBuf;
use tokio::fs;

impl Config {
    async fn new(
        config_dir: PathBuf,
        temp_dir: PathBuf,
        data_path: PathBuf,
        rice_path: PathBuf,
        package_managers: PathBuf,
    ) -> color_eyre::Result<Config> {
        let config = Config {
            config_dir,
            temp_dir,
            data_path,
            rice_path,
            package_managers,
            package_manager_setting: PackageManagerSetting::None,
        };

        config.save().await?;

        Ok(config)
    }

    pub async fn load() -> color_eyre::Result<Config> {
        let config_dir = get_config_dir();
        let data_path = get_data_path();
        let package_managers = get_data_path().join("package_manager");
        let rice_path = get_data_path().join("rice");
        let temp_dir = get_temp_dir();

        let (res1, res2, res3, res4, res5) = tokio::join!(
            tokio::fs::create_dir_all(config_dir.clone()),
            tokio::fs::create_dir_all(data_path.clone()),
            tokio::fs::create_dir_all(package_managers.clone()),
            tokio::fs::create_dir_all(rice_path.clone()),
            tokio::fs::create_dir_all(temp_dir.clone()),
        );
        res1?;
        res2?;
        res3?;
        res4?;
        res5?;

        let config_path = config_dir.join("config.json");

        if !fs::try_exists(&config_dir).await? {
            return Self::new(config_dir, temp_dir, data_path, rice_path, package_managers).await;
        };

        let content = fs::read_to_string(&config_path).await?;
        let mut config = serde_json::from_str::<Config>(&content)?;

        match &config.package_manager_setting {
            PackageManagerSetting::None => {
                let (current, path) =
                    PackageManager::find_installed_manager(&package_managers).await?;
                config.package_manager_setting = PackageManagerSetting::Current {
                    preferred: path,
                    current,
                };
            }
            PackageManagerSetting::Preferred(preferred) => {
                PackageManager::load(
                    &config,
                    PackageManagerSource::Local {
                        path: preferred.clone(),
                    },
                )
                .await?;

                config.package_manager_setting = PackageManagerSetting::Current {
                    preferred: preferred.clone(),
                    current: PackageManager::default(),
                };
            }
            PackageManagerSetting::Current { .. } => {}
        }

        Ok(config)
    }

    pub async fn save(&self) -> color_eyre::Result<()> {
        let config_path = self.config_dir.join("config.json");
        fs::write(config_path, serde_json::to_string_pretty(self)?)
            .await
            .wrap_err("Failed to save configuration file")
    }
}
