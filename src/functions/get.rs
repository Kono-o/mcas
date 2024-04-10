use anyhow::Result;
use concat_string::concat_string;
use futures::StreamExt;
use indicatif::ProgressBar;
use std::path::{Path, PathBuf};

use crate::log;

const URL: &str = "https://github.com/InventivetalentDev/minecraft-assets/zipball/refs/heads/";

pub async fn try_get(version: &str, dir: &PathBuf) -> Result<()> {
    log::msg(&concat_string!("fetching [", version, "]..."));

    let url: String = format!("{}{}", URL, version);
    let save_dir: PathBuf = dir.join(Path::new(&concat_string!(version, ".zip")));
    let bar = ProgressBar::new(32000);

    let mut file = tokio::fs::File::create(save_dir).await?;
    let mut byte_stream = reqwest::get(&url).await?.bytes_stream();

    while let Some(item) = byte_stream.next().await {
        bar.inc(1);
        tokio::io::copy(&mut item?.as_ref(), &mut file).await?;
    }
    bar.finish_and_clear();
    Ok(())
}
