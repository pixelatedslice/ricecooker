use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PackageFormat {
    pub versioned: String,
    pub unversioned: String,
}
