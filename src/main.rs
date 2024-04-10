use indicatif;
use std::path::{Path, PathBuf};
use std::{fs::File, io::copy};

const URL: &str = "https://github.com/InventivetalentDev/minecraft-assets/zipball/refs/heads/";

enum McasError {
    InvalidCurDir,
    InvalidFunc,
    MissingFunc,
    InvalidVer,
    MissingVer,
}
struct Args {
    cur_dir: PathBuf,
    func: String,
    ver: String,
}
impl Args {
    pub fn parse() -> Result<Self, McasError> {
        let cur_dir: PathBuf;
        match std::env::current_dir() {
            Ok(dir) => cur_dir = dir,
            Err(_) => {
                return Err(McasError::InvalidCurDir);
            }
        }

        let func: String;
        match std::env::args().nth(1) {
            Some(f) => func = f,
            _ => return Err(McasError::MissingFunc),
        }

        let ver: String;
        match std::env::args().nth(2) {
            Some(v) => ver = v,
            _ => return Err(McasError::MissingVer),
        }
        Ok(Self { cur_dir, func, ver })
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

fn invalid_msg() {
    println!("invalid format, try <func> <version>")
}
