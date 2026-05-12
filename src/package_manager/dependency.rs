use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Dependency {
    #[serde(rename = "remote", untagged)]
    RemoteInstaller {
        url: Url,
        sha256: Option<String>,
        args: Vec<String>,
        check_command: Option<String>,
    },

    #[serde(rename = "command", untagged)]
    Command {
        command: String,
        unless: Option<String>,
    },

    #[serde(rename = "package", untagged)]
    PackageManager {
        name: String,
        version: Option<String>,
    },
}
