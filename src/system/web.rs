use futures_util::StreamExt;
use std::path::Path;
use tokio::{fs, io};
use tracing::{debug, trace};
use url::Url;

pub async fn stream_file<P: AsRef<Path>>(url: &Url, target_path: P) -> color_eyre::Result<()> {
    let target_path = target_path.as_ref();
    debug!(%url, ?target_path, "Streaming file");
    let client = reqwest::Client::new();

    let response = client.get(url.as_str()).send().await?.error_for_status()?;
    let content_length = response.content_length();
    trace!(?content_length, "Response received");

    let mut file = fs::File::create(target_path).await?;

    let mut stream = response.bytes_stream();

    let mut downloaded: u64 = 0;
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        io::AsyncWriteExt::write_all(&mut file, &chunk).await?;
        downloaded += chunk.len() as u64;
        trace!(downloaded, "Downloaded chunk");
    }

    debug!(downloaded, "File streaming completed");
    Ok(())
}

pub fn get_file_name_from_url(url: &Url) -> Option<&str> {
    url.path_segments()
        .and_then(|mut segments| segments.next_back())
        .filter(|&last| !last.is_empty())
}
