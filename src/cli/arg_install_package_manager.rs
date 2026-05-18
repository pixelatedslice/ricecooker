use crate::cli::{Cli, InputSource};
use crate::config::app::Config;
use crate::package_manager::PackageManager;
use crate::source::Source;
use color_eyre::eyre::bail;

impl Cli {
    pub async fn install_package_manager(&self, config: &mut Config) -> color_eyre::Result<()> {
        let Some(source) = &self.install_package_manager else {
            bail!("No package manager source specified.")
        };

        let source = match source {
            InputSource::Path(path) => Source::Local { path: path.clone() },
            InputSource::Url(url) => Source::Remote { url: url.clone() },
        };

        let package_manager = PackageManager::load(config, source).await?;

        config.set_package_manager(package_manager.clone()).await
    }
}
