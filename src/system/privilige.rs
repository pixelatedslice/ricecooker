use tracing::{debug, trace};
use which::which;

#[cfg(unix)]
fn is_root() -> bool {
    let uid = unsafe { libc::getuid() };
    trace!(uid, "Checking if root");
    uid == 0
}

#[cfg(unix)]
fn get_elevation_command() -> String {
    debug!("Searching for elevation command");
    let cmd = ["sudo", "doas", "run0", "pkexec"]
        .into_iter()
        .find(|&binary_name| {
            let ok = which(binary_name).is_ok();
            trace!(binary_name, ok, "Checking elevation command");
            ok
        })
        .expect("No elevation command found")
        .into();
    debug!(elevation_command = %cmd, "Found elevation command");
    cmd
}
