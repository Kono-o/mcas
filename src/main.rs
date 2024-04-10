use indicatif;
use std::path::{Path, PathBuf};
use std::{fs::File, io::copy};

const URL: &str = "https://github.com/InventivetalentDev/minecraft-assets/zipball/refs/heads/";

enum McasError {
    InvalidDir,
    InvalidFunc,
    MissingFunc,
    InvalidVer,
    MissingVer,
}
struct Args {
    dir: PathBuf,
    func: String,
    ver: String,
}
impl Args {
    pub fn parse() -> Result<Self, McasError> {
        return match (
            std::env::current_dir(),
            std::env::args().nth(1),
            std::env::args().nth(2),
        ) {
            (Ok(dir), Some(func), Some(ver)) => Ok(Self { dir, func, ver }),
            (Ok(_), Some(_), None) => Err(McasError::MissingVer),
            (Ok(_), None, None) => Err(McasError::InvalidFunc),
            _ => Err(McasError::InvalidDir),
        };
    }
}
fn main() {
    let args = Args::parse();
}
fn try_download(version: &String, dir: &PathBuf) {
    let url: String = format!("{}{}", URL, version);
    let save_dir: PathBuf = dir.join(Path::new(format!("{}.zip", version)));
    let mut response = reqwest::blocking::get(url).unwrap();
    if response.status().as_u16() >= 300 {
        return;
    }
    let mut file = File::create(save_dir).unwrap();
    copy(&mut response, &mut file).unwrap();
}
