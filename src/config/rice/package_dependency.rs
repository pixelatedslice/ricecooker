use crate::package_manager::PackageManager;
use crate::ErrorMessages;
use color_eyre::eyre::bail;
use tracing::{debug, warn};

pub async fn install_package_dependencies(
    package_manager: &PackageManager,
    dependencies: &[(&String, &Option<String>)],
) -> color_eyre::Result<()> {
    debug!("Installing {} package dependencies", dependencies.len());
    let mut errors = ErrorMessages::new();

    if let Err(error) = package_manager.install(dependencies).await {
        warn!(%error, "Package dependency installation failed");
        errors.push(format!("Could not install package dependency: {}", error));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        bail!(errors.join("; "))
    }
}
