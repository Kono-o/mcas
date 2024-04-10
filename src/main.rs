#[path = "functions/get.rs"]
mod get;
#[path = "functions/help.rs"]
mod help;

mod args;
pub mod log;

use crate::args::{parse, run};

#[tokio::main]
async fn main() {
    run(parse()).await;
}
