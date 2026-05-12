extern crate core;

use crate::config::app::config::Config;
use tracing_appender::non_blocking::WorkerGuard;

mod config;
mod logging;
mod operating_system;
mod package_manager;
mod system;

type ErrorMessages = Vec<String>;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let config = Config::load().await?;

    let _guard: WorkerGuard = logging::setup(false)?;

    Ok(())
}
