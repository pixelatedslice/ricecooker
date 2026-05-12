use futures_util::StreamExt;
use std::path::PathBuf;
use tokio::{fs, io};
use url::Url;

pub async fn stream_file(url: &Url, target_path: &PathBuf) -> color_eyre::Result<()> {
    let client = reqwest::Client::new();

    let response = client.get(url.as_str()).send().await?.error_for_status()?;

    let mut file = fs::File::create(target_path).await?;

    let mut stream = response.bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        io::AsyncWriteExt::write_all(&mut file, &chunk).await?
    }

    Ok(())
}

pub fn get_file_name_from_url(url: &Url) -> Option<&str> {
    url.path_segments()
        .and_then(|mut segments| segments.next_back())
        .filter(|&last| !last.is_empty())
}
