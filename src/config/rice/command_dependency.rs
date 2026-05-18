use crate::system::cmd::spawn_shell;
use crate::ErrorMessages;
use color_eyre::eyre::bail;
use tracing::{debug, info, warn};

pub async fn install_command_dependencies(
    dependencies: &[(&Vec<String>, &Option<Vec<String>>)],
) -> color_eyre::Result<()> {
    debug!("Installing {} command dependencies", dependencies.len());
    let mut errors = ErrorMessages::new();

    for (command, unless) in dependencies {
        if let Err(error) = install_single_command(command, unless).await {
            warn!(?command, %error, "Command dependency installation failed");
            errors.push(format!("Could not install command dependency: {}", error));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        bail!(errors.join("; "))
    }
}

pub async fn install_single_command(
    command: &Vec<String>,
    unless: &Option<Vec<String>>,
) -> color_eyre::Result<()> {
    if command.is_empty() {
        return Ok(());
    }

    if let Some(unless) = unless {
        debug!(?unless, "Checking if command needs to be run (unless)");
        let status = spawn_shell().args(unless).status()?;
        if status.success() {
            debug!("'unless' command succeeded, skipping main command");
            return Ok(());
        }
    }

    info!("Running command: {}", command.join(" "));
    let status = spawn_shell().args(command).status()?;

    if !status.success() {
        bail!("Installation failed for {}: {}", command.join(" "), status)
    }

    debug!("Command executed successfully");
    Ok(())
}
