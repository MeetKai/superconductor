mod lib;

#[tokio::main]
async fn main() {
    env_logger::init();
    lib::run().await;
}
