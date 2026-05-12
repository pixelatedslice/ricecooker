pub mod dependency;
pub mod implementation;

use crate::operating_system::OperatingSystem;
use aho_corasick::AhoCorasick;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::LazyLock;
use url::Url;

pub static INSTALL_COMMAND_AC: LazyLock<AhoCorasick> =
    LazyLock::new(|| AhoCorasick::new(["{pkgs}"]).unwrap());
pub static VERSIONED_PACKAGE_FORMAT_AC: LazyLock<AhoCorasick> =
    LazyLock::new(|| AhoCorasick::new(["{pkg}", "{ver}"]).unwrap());
pub static UNVERSIONED_PACKAGE_FORMAT_AC: LazyLock<AhoCorasick> =
    LazyLock::new(|| AhoCorasick::new(["{pkg}"]).unwrap());

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PackageManager {
    #[serde(flatten)]
    pub operating_systems: Vec<OperatingSystem>,
    pub binary_name: String,
    pub update_command: String,
    pub install_command: String,
    pub package_format: PackageFormat,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PackageFormat {
    pub versioned: String,
    pub unversioned: String,
}

pub enum PackageManagerSource {
    Local { path: PathBuf },
    Remote { url: Url },
}
