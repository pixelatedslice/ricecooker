use crate::config::rice::modified_file::ModifiedFile;
use crate::operating_system::OperatingSystem;
use crate::package_manager::dependency::Dependency;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rice {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub operating_system: OperatingSystem,
    pub files: Vec<ModifiedFile>,
    pub dependencies: HashMap<OperatingSystem, Vec<Dependency>>,
}
