use crate::cli::Cli;
use crate::config::app::Config;
use color_eyre::eyre::bail;

impl Cli {
    pub(crate) fn set_data_path(&self, config: &mut Config) -> color_eyre::Result<()> {
        let Some(path) = &self.set_data_path else {
            bail!("No data path specified.")
        };

        config.data_path = path.clone();

        Ok(())
    }
}
