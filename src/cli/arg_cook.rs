use crate::cli::Cli;
use crate::config::app::Config;
use crate::config::rice::config::Rice;
use color_eyre::eyre::bail;

impl Cli {
    async fn cook(&self, config: &mut Config) -> color_eyre::Result<()> {
        let Some(source) = &self.cook else {
            bail!("No rice config source specified.")
        };

        Rice::apply(config, source).await?;

        Ok(())
    }
}
