mod args;
mod log;

use crate::args::Args;
use concat_string::concat_string;
use indicatif;
use std::path::{Path, PathBuf};
use std::{fs::File, io::copy};

const URL: &str = "https://github.com/InventivetalentDev/minecraft-assets/zipball/refs/heads/";

fn main() {
    let args_result = Args::parse();
    match args_result {
        Err(err) => err.handle(),
        Ok(a) => match a {
            Args::Two(..) => println!("2"),
            Args::One(..) => println!("1"),
        },
    }
}

fn try_download(version: &str, dir: &PathBuf) {
    let url: String = format!("{}{}", URL, version);
    let save_dir: PathBuf = dir.join(Path::new(&concat_string!("{}.zip", version)));
    let mut response = reqwest::blocking::get(url).unwrap();
    if response.status().as_u16() >= 300 {
        return;
    }
    let mut file = File::create(save_dir).unwrap();
    copy(&mut response, &mut file).unwrap();
}
