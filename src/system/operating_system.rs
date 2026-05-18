use crate::package_manager::dependency::Dependency;
use color_eyre::eyre::bail;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, trace};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum OperatingSystem {
    Unix,
    Windows,
    MacOS,
    Bsd,
}

impl OperatingSystem {
    pub fn get_current_os() -> color_eyre::Result<OperatingSystem> {
        let os = std::env::consts::OS;
        trace!(%os, "Determining current operating system");

        if os.contains("windows") {
            return Ok(OperatingSystem::Windows);
        }
        if os.contains("linux") {
            return Ok(OperatingSystem::Unix);
        }
        if os.contains("macos") {
            return Ok(OperatingSystem::MacOS);
        }
        if os.contains("bsd") {
            return Ok(OperatingSystem::Bsd);
        }

        bail!("The current os ({}) is not supported", os)
    }

    pub fn get_dependencies_for_current_os(
        dependencies: &HashMap<OperatingSystem, Vec<Dependency>>,
    ) -> color_eyre::Result<&[Dependency]> {
        let os = Self::get_current_os()?;
        debug!(?os, "Getting dependencies for OS");
        let dependencies = dependencies
            .get(&os)
            .map_or(&[] as &[Dependency], Vec::as_slice);

        trace!(count = dependencies.len(), "Found dependencies");
        Ok(dependencies)
    }
}
