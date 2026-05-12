use crate::package_manager::PackageManager;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(skip)]
    pub config_dir: PathBuf,

    #[serde(skip)]
    pub temp_dir: PathBuf,

    #[serde(skip)]
    pub data_path: PathBuf,

    #[serde(skip)]
    pub rice_path: PathBuf,

    #[serde(skip)]
    pub package_managers: PathBuf,

    #[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
    pub package_manager_setting: PackageManagerSetting,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PackageManagerSetting {
    None,
    Preferred(PathBuf),
    Current {
        preferred: PathBuf,
        #[serde(skip)]
        current: PackageManager,
    },
}

impl Default for PackageManagerSetting {
    fn default() -> Self {
        PackageManagerSetting::None
    }
}

fn serialize<S>(value: &PackageManagerSetting, serializer: S) -> color_eyre::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let path = match value {
        PackageManagerSetting::Preferred(p) => Some(p),
        PackageManagerSetting::Current { preferred, .. } => Some(preferred),
        PackageManagerSetting::None => None,
    };

    match path {
        Some(path) => path.serialize(serializer),
        None => serializer.serialize_none(),
    }
}

fn deserialize<'de, D>(deserializer: D) -> color_eyre::Result<PackageManagerSetting, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<PathBuf>::deserialize(deserializer)? {
        Some(path) => Ok(PackageManagerSetting::Preferred(path)),
        None => Ok(PackageManagerSetting::None),
    }
}
