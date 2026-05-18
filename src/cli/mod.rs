pub mod arg_cook;
pub mod arg_install_package_manager;
pub mod arg_set_data_path;

use crate::config::app::Config;
use crate::config::rice::config::Rice;
use clap::ArgAction;
use clap::Parser;
use std::path::PathBuf;
use tracing::{debug, info};
use url::Url;

#[derive(Debug, Parser)]
#[command(author, version, about = "A custom package manager CLI")]
pub struct Cli {
    #[arg(
        short = 'i',
        long = "install-package-manager",
        value_parser = parse_input_source
    )]
    install_package_manager: Option<InputSource>,

    #[arg(short = 'd', long = "set-data-path")]
    set_data_path: Option<PathBuf>,

    #[arg(
        short = 'c',
        long = "cook",
        value_parser = parse_input_source
    )]
    cook: Option<InputSource>,

    #[arg(short = 'v', long = "verbose", action = ArgAction::SetTrue)]
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub enum InputSource {
    Path(PathBuf),
    Url(Url),
}

impl Cli {
    pub async fn run(&self, config: &mut Config) -> color_eyre::Result<()> {
        if let Some(path) = &self.set_data_path {
            info!("Setting data path to {:?}", path);
            self.set_data_path(config)?;
        }

        if let Some(package_manager) = &self.install_package_manager {
            info!("Installing package manager from {:?}", package_manager);
            self.install_package_manager(config).await?;
        }

        if self.set_data_path.is_some() || self.install_package_manager.is_some() {
            debug!("Saving configuration after updates");
            config.save().await?;
        }

        if let Some(rice_source) = &self.cook {
            info!("Cooking rice from {:?}", rice_source);
            Rice::apply(config, rice_source).await?;
        };

        Ok(())
    }
}

fn parse_input_source(s: &str) -> color_eyre::Result<InputSource, String> {
    if let Ok(url) = Url::parse(s)
        && (url.scheme() == "http" || url.scheme() == "https")
    {
        Ok(InputSource::Url(url))
    } else {
        Ok(InputSource::Path(PathBuf::from(s)))
    }
}
