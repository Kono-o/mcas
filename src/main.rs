mod args;
mod func;

#[tokio::main]
async fn main() {
    args::go().await;
}
