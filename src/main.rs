mod func;
mod mcas;

#[tokio::main]
async fn main() {
    mcas::run().await;
}
