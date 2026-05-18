use std::process::Command;
use tracing::{debug, error, info};

#[cfg(unix)]
pub fn spawn_shell() -> Command {
    debug!("Spawning sh shell");
    let mut cmd = Command::new("sh");
    cmd.arg("-c");
    cmd
}

#[cfg(windows)]
pub fn spawn_shell() -> Command {
    debug!("Spawning cmd shell");
    let mut cmd = Command::new("cmd");
    cmd.arg("/C");
    cmd
}

pub fn run_command_in_shell<S: AsRef<str>>(command_ref: S) -> color_eyre::Result<()> {
    let command = command_ref.as_ref();
    info!("Executing: {}", command);

    let mut shell = spawn_shell();
    shell.arg(command);

    let status = shell.status()?;

    if !status.success() {
        error!("Command failed: {} (status: {})", command, status);
        return Err(color_eyre::eyre::eyre!(
            "Command exited with status: {}",
            status
        ));
    }

    debug!("Command completed successfully");
    Ok(())
}
