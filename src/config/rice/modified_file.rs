use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ModifiedFile {
    Content {
        path: PathBuf,
        content: String,
        sha256: Option<String>,
    },
    Remote {
        path: PathBuf,
        url: Url,
        sha256: Option<String>,
    },
}

impl ModifiedFile {
    pub fn content(&self) -> Option<&String> {
        match self {
            ModifiedFile::Content { content, .. } => Some(content),
            ModifiedFile::Remote { .. } => None,
        }
    }
}
