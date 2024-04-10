use concat_string::concat_string;
use std::fs::File;
use std::io::copy;
use std::path::{Path, PathBuf};

const URL: &str = "https://github.com/InventivetalentDev/minecraft-assets/zipball/refs/heads/";

pub fn try_get(v: &str, dir: &PathBuf) {
    println!("{} {:?}", v, dir);
}

/*pub fn try_get(version: &str, dir: &PathBuf) {
    let url: String = format!("{}{}", URL, version);
    let save_dir: PathBuf = dir.join(Path::new(&concat_string!("{}.zip", version)));
    let mut response = reqwest::blocking::get(url).unwrap();
    if response.status().as_u16() >= 300 {
        return;
    }
    let mut file = File::create(save_dir).unwrap();
    copy(&mut response, &mut file).unwrap();
}*/
