use std::path::PathBuf;
use url::Url;

#[derive(Debug, Clone)]
pub enum Source {
    Local { path: PathBuf },
    Remote { url: Url },
}
