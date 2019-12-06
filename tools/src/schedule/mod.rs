use rebuilderd_common::errors::*;
use reqwest::Client;
use std::fs;

pub async fn url_or_path(client: &Client, path: &str) -> Result<Vec<u8>> {
    let bytes = if path.starts_with("https://") || path.starts_with("http://") {
        info!("Downloading {:?}...", path);
        client.get(path)
            .send()
            .await?
            .bytes()
            .await?
            .to_vec()
    } else {
        info!("Reading {:?}...", path);
        fs::read(path)?
    };

    Ok(bytes)
}

pub mod archlinux;
pub mod debian;