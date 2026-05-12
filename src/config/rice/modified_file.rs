use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ModifiedFile {
    Content { path: PathBuf, content: String },
    Remote { path: PathBuf, url: Url },
}
