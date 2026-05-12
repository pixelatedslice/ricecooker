use which::which;

#[cfg(unix)]
fn is_root() -> bool {
    let uid = unsafe { libc::getuid() };
    uid == 0
}

#[cfg(unix)]
fn get_elevation_command() -> String {
    ["sudo", "doas", "run0", "pkexec"]
        .into_iter()
        .find(|&binary_name| which(binary_name).is_ok())
        .expect("No elevation command found")
        .into()
}
