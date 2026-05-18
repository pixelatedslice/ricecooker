use crate::package_manager::PackageManager;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub enum PackageManagerSetting {
    #[default]
    None,
    Preferred(PathBuf),
    Current {
        preferred: PathBuf,
        #[serde(skip)]
        current: PackageManager,
    },
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
