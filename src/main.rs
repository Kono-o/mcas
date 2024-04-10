use indicatif;
use std::path::{Path, PathBuf};
use std::{fs::File, io::copy};

const URL: &str = "https://github.com/InventivetalentDev/minecraft-assets/zipball/refs/heads/";

fn main() {
    let cur_dir: PathBuf;
    match std::env::current_dir() {
        Ok(dir) => cur_dir = dir,
        Err(err) => {
            println!("{}", err);
            return;
        }
    }
    let func: String;
    let ver: String;
    match (std::env::args().nth(1), std::env::args().nth(2)) {
        (Some(f), Some(v)) => {
            func = f;
            ver = v
        }
        _ => {
            invalid_msg();
            return;
        }
    }
    match func.as_str() {
        "get" => try_download(&ver, &cur_dir),
        _ => {
            invalid_msg();
            return;
        }
    }
}
fn try_download(version: &String, dir: &PathBuf) {
    let url: String = format!("{}{}", URL, version);
    let save_dir: PathBuf = dir.join(Path::new(format!("{}.zip",version)));
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
