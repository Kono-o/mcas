use anyhow::Result;
use concat_string::concat_string;
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
use std::path::{Path, PathBuf};

use crate::log;

const URL: &str = "https://github.com/InventivetalentDev/minecraft-assets/zipball/refs/heads/";

pub async fn try_get(version: &str, dir: &PathBuf) -> Result<()> {
    log::msg(&concat_string!("fetching [", version, "]..."));

    let url: String = format!("{}{}", URL, version);
    let save_dir: PathBuf = dir.join(Path::new(&concat_string!(version, ".zip")));
    let bar = ProgressBar::new(32000);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] [{elapsed_precise}]",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("=>-"),
    );

    let mut file = tokio::fs::File::create(save_dir).await?;
    let mut byte_stream = reqwest::get(&url).await?.bytes_stream();

    while let Some(item) = byte_stream.next().await {
        bar.inc(1);
        match item {
            Ok(ref r) =>  {
                b"404: Not Found" => println!("404"),
                _ => println!("_"),
            },
            Err(_) => (),
        }

        tokio::io::copy(&mut item?.as_ref(), &mut file).await?;
    }
    bar.finish_and_clear();
    Ok(())
}
