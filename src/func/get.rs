use crate::func;
use anyhow::Result;
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::path::PathBuf;
use std::time::Duration;

const URL: &str = "https://github.com/InventivetalentDev/minecraft-assets/zipball/refs/heads/";

pub async fn get(version: &str, dir: &mut PathBuf, out: Option<&PathBuf>) -> Result<()> {
    let out = out.unwrap_or(dir);
    let mut dir = dir.clone();
    let mut final_out = out.clone();
    let out_str = out.to_str().unwrap_or("");
    if out_str.chars().nth(0).unwrap_or(' ') == '+' {
        dir.extend(out);
        final_out = dir;
    }
    get_dir(version, &final_out).await
}

pub async fn get_dir(version: &str, out: &PathBuf) -> Result<()> {
    let url = format!("{}{}", URL, version);
    let pack = format!("{version}.zip");
    let filepath = out.join(&pack);
    let out_str = out.to_string_lossy();

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("fetching pack for v{msg} {spinner}")?.tick_chars(r"-\\|/ "),
    );
    spinner.set_message(version.to_string());
    spinner.enable_steady_tick(Duration::from_millis(80));

    let client = Client::new();
    let resp = client.get(&url).send().await?;

    if resp.status() == 404 {
        spinner.finish_and_clear();
        return Err(anyhow::anyhow!(format!("v{version} not found!")));
    }

    spinner.finish_and_clear();

    let total_size = resp.content_length();
    let mut file = tokio::fs::File::create(&filepath).await?;
    let mut stream = resp.bytes_stream();

    let bar = match total_size {
        Some(size) => {
            let bar = ProgressBar::new(size);
            bar.set_style(
                ProgressStyle::with_template(
                    "saving {msg} {bar:40.cyan/blue} {percent:>3}% [{bytes:>8}]",
                )?
                .progress_chars("=>-"),
            );
            bar.set_message(format!("v{}", version));
            Some(bar)
        }
        None => {
            let bar = ProgressBar::new_spinner();
            bar.set_style(ProgressStyle::with_template("saving {msg} [{bytes:>8}]")?);
            bar.enable_steady_tick(Duration::from_millis(100));
            bar.set_message(format!("v{}", version));
            Some(bar)
        }
    };

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        tokio::io::copy(&mut chunk.as_ref(), &mut file).await?;
        if let Some(ref bar) = bar {
            bar.inc(chunk.len() as u64);
        }
    }

    if let Some(ref bar) = bar {
        bar.finish_and_clear();
    }

    func::msg_ok(&format!("{pack} saved to ({})!", out_str));
    Ok(())
}
