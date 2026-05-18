extern crate core;

use crate::cli::Cli;
use crate::config::app::Config;
use clap::Parser;
use tracing_appender::non_blocking::WorkerGuard;

mod cli;
mod config;
mod logging;
mod package_manager;
pub mod source;
mod system;

type ErrorMessages = Vec<String>;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let _guard: WorkerGuard = logging::setup(cli.verbose)?;

    let mut config = Config::load().await?;

    cli.run(&mut config).await?;

    Ok(())
}
