use crate::config::app::Config;
use crate::config::rice::command_dependency::{
    install_command_dependencies, install_single_command,
};
use crate::config::rice::package_dependency::install_package_dependencies;
use crate::config::rice::remote_dependency::{install_remote_dependencies, install_single_remote};
use serde::{Deserialize, Serialize};
use tokio::join;
use tracing::{debug, info};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Dependency {
    #[serde(rename = "remote", untagged)]
    RemoteInstaller {
        url: Url,
        sha256: Option<String>,
        args: Vec<String>,
        check_command: Option<Vec<String>>,
    },

    #[serde(rename = "command", untagged)]
    Command {
        command: Vec<String>,
        unless: Option<Vec<String>>,
    },

    #[serde(rename = "package", untagged)]
    Package {
        name: String,
        version: Option<String>,
    },
}

impl Dependency {
    pub async fn install_multiple(
        config: &mut Config,
        dependencies: &[Dependency],
    ) -> color_eyre::Result<()> {
        info!("Installing {} dependencies", dependencies.len());
        let (mut packages, mut commands, mut remotes) = (Vec::new(), Vec::new(), Vec::new());

        for dependency in dependencies {
            match dependency {
                Dependency::Package { name, version } => packages.push((name, version)),
                Dependency::Command { command, unless } => commands.push((command, unless)),
                Dependency::RemoteInstaller {
                    url,
                    sha256,
                    args,
                    check_command,
                } => remotes.push((url, sha256, args, check_command)),
            }
        }

        let package_manager = config.get_package_manager().await?.clone();

        if !packages.is_empty() {
            debug!("Installing {} package dependencies", packages.len());
            install_package_dependencies(&package_manager, &packages).await?;
        }

        debug!("Installing command and remote dependencies");
        let (res1, res2) = join!(
            install_command_dependencies(&commands),
            install_remote_dependencies(config, &remotes)
        );

        res1?;
        res2?;

        info!("All dependencies installed");
        Ok(())
    }

    pub async fn install(&self, config: &mut Config) -> color_eyre::Result<()> {
        match self {
            Dependency::RemoteInstaller {
                url,
                sha256,
                args,
                check_command,
            } => {
                info!("Installing remote dependency: {}", url);
                install_single_remote(config, url, sha256, args, check_command).await
            }
            Dependency::Command { command, unless } => {
                info!("Running command dependency: {:?}", command);
                install_single_command(command, unless).await
            }
            Dependency::Package { name, version } => {
                info!("Installing package dependency: {} (v{:?})", name, version);
                let package_manager = config.get_package_manager().await?.clone();
                install_package_dependencies(&package_manager, &[(name, version)]).await
            }
        }
    }
}
