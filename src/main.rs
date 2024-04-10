use indicatif;
use std::path::PathBuf;
use std::{fs, fs::File, io::copy};

const URL: &str = "https://github.com/InventivetalentDev/minecraft-assets/zipball/refs/heads/";
const SAVE_PATH: &str = "/media/kono/HDD/Coding/mcas/zips";

fn main() {
    let cur_dir: PathBuf;
    match std::env::current_dir() {
        Ok(dir) => cur_dir = dir,
        Err(err) => {
            println!("{}", err);
            return;
        }
    }
    let func;
    let ver;
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
        "get" => try_download(ver),
        _ => {
            invalid_msg();
            return;
        }
    }
}
fn try_download(version: String) {
    match fs::create_dir(SAVE_PATH) {
        Ok(_) => println!("created {}", SAVE_PATH),
        _ => (),
    }

    let url: String = format!("{}{}", URL, version);
    let file_dir: String = format!("{}{}.zip", SAVE_PATH, version);
    let mut response = reqwest::blocking::get(url).unwrap();
    if response.status().as_u16() >= 300 {
        return;
    }
    let mut file = File::create(file_dir).unwrap();
    copy(&mut response, &mut file).unwrap();
}

fn invalid_msg() {
    println!("invalid format, try <func> <version>")
}
