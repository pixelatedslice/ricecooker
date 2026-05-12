use std::process::Command;
use tracing::error;

#[cfg(unix)]
pub fn spawn_shell() -> Command {
    let mut cmd = Command::new("sh");
    cmd.arg("-c");
    cmd
}

#[cfg(windows)]
pub fn spawn_shell() -> Command {
    let mut cmd = Command::new("cmd");
    cmd.arg("/C");
    cmd
}

pub fn run_command_in_shell<S: AsRef<str>>(command_ref: S) -> color_eyre::Result<()> {
    let command = command_ref.as_ref();

    let mut shell = spawn_shell();
    shell.arg(command);

    let status = shell.status()?;

    if !status.success() {
        error!("Failed to run command: {}", command);
        return Err(color_eyre::eyre::eyre!(
            "Command exited with status: {}",
            status
        ));
    }

    Ok(())
}
