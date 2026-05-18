use crate::config::app::Config;
use crate::system::cmd::spawn_shell;
use crate::system::io::{check_hash, cleanup_path};
use crate::system::web::{get_file_name_from_url, stream_file};
use crate::ErrorMessages;
use color_eyre::eyre::{bail, eyre};
use std::ffi::OsStr;
use std::path::PathBuf;
use tokio::fs;
use tracing::{debug, info, trace, warn};
use url::Url;

pub async fn install_remote_dependencies(
    config: &Config,
    dependencies: &[(&Url, &Option<String>, &Vec<String>, &Option<Vec<String>>)],
) -> color_eyre::Result<()> {
    debug!("Installing {} remote dependencies", dependencies.len());
    let mut errors = ErrorMessages::new();

    for (url, sha256, args, check_command) in dependencies {
        if let Err(err) = install_single_remote(config, url, sha256, args, check_command).await {
            warn!(%url, %err, "Remote dependency installation failed");
            errors.push(format!("Could not install remote dependency: {}", err));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        bail!(errors.join("; "))
    }
}

pub async fn install_single_remote(
    config: &Config,
    url: &Url,
    sha256: &Option<String>,
    args: &[String],
    check_command: &Option<Vec<String>>,
) -> color_eyre::Result<()> {
    if let Some(cmd) = check_command {
        debug!(?cmd, "Checking if remote dependency needs to be installed");
        if run_check_command(cmd)? {
            debug!(%url, "Check command succeeded, skipping installation");
            return Ok(());
        }
    }

    let file_name = get_file_name_from_url(url)
        .ok_or_else(|| eyre!("Could not get file name from url {}", url))?;
    let temp_file = config.temp_dir.join(file_name);

    info!("Installing remote dependency: {}", url);
    let result = execute_installation(&temp_file, url, sha256, args).await;

    trace!(?temp_file, "Removing temporary installer file");
    if let Err(err) = fs::remove_file(&temp_file).await {
        warn!(
            "Could not remove temporary file {}: {}",
            temp_file.display(),
            err
        );
    }

    result
}

async fn execute_installation(
    path: &PathBuf,
    url: &Url,
    sha256: &Option<String>,
    args: &[String],
) -> color_eyre::Result<()> {
    debug!(%url, ?path, "Downloading installer");
    stream_file(url, path).await.map_err(|e| {
        cleanup_path(path);
        eyre!("Download failed for {}: {}", url, e)
    })?;

    if let Some(expected) = sha256 {
        debug!("Verifying installer SHA256");
        check_hash(path, expected, url).await?;
    }

    info!("Running installer: {} {}", path.display(), args.join(" "));
    let status = spawn_shell().arg(path).args(args).status()?;

    if !status.success() {
        bail!("Installation failed for {}: {}", url, status);
    }

    debug!("Installer finished successfully");
    Ok(())
}

async fn try_removing_file<P: AsRef<std::path::Path>>(
    error_messages: &mut ErrorMessages,
    file_path: P,
) {
    let file_path = file_path.as_ref();
    if let Err(error) = fs::remove_file(&file_path).await {
        error_messages.push(format!(
            "Could not remove file {}. Error: {}",
            file_path.display(),
            error
        ));
    }
}

fn run_check_command<S: AsRef<OsStr>>(check_command: &Vec<S>) -> color_eyre::Result<bool> {
    let result = spawn_shell()
        .args(check_command)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success())?;

    Ok(result)
}
