#[path = "functions/get.rs"]
mod get;
#[path = "functions/help.rs"]
mod help;

mod args;
mod log;

use crate::args::{parse, run};

fn main() {
    run(parse());
}
