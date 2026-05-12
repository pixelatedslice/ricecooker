use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum OperatingSystem {
    Unix,
    Windows,
    MacOS,
    BSD,
}
