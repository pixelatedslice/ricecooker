use crate::system::directories::get_home_env;
use std::env;
use std::path::PathBuf;
use tracing::metadata::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Layer};

pub fn setup(verbose_console: bool) -> Result<WorkerGuard, color_eyre::Report> {
    let path: PathBuf = get_log_file_path();
    std::fs::create_dir_all(&path).expect("Failed to create log directory");

    let file_appender = tracing_appender::rolling::daily(path, "ricecooker.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_file(true)
        .with_ansi(false)
        .with_writer(non_blocking)
        .with_line_number(true)
        .with_target(false)
        .with_thread_ids(true)
        .with_filter(LevelFilter::TRACE);

    let console_layer = fmt::layer()
        .with_ansi(true)
        .with_target(true)
        .with_writer(std::io::stdout)
        .with_filter(
            EnvFilter::builder()
                .with_default_directive(
                    (if verbose_console {
                        LevelFilter::TRACE
                    } else {
                        LevelFilter::INFO
                    })
                    .into(),
                )
                .from_env_lossy(),
        );

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .with(ErrorLayer::default())
        .init();

    Ok(guard)
}

fn get_log_file_path() -> PathBuf {
    if let Ok(env_path) = env::var("XDG_STATE_HOME") {
        [env_path.as_str(), "ricecooker"].iter().collect()
    } else {
        [get_home_env().as_str(), ".local", "state", "ricecooker"]
            .iter()
            .collect()
    }
}
