use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiceConfig {
    name: String,
    description: String,
    version: String,
    author: String,
    updated_at: String,
    url: String,
    #[serde(flatten, default)]
    files: Vec<ModifiedFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifiedFile {
    path: PathBuf,
    #[serde(flatten)]
    source: FileSource,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileSource {
    Content { content: String },
    External { source: String },
}

impl RiceConfig {
    pub fn from(content: String) -> Result<Self, serde_json::Error> {
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }
}
