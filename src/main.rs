use clap::Parser;
use indicatif;

#[derive(Parser)]
struct Args {
    function: String,
    version: String,
}

fn main() {
    let args = Args::parse();

    println!("pattern: {:?}, path: {:?}", args.function, args.version);
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
