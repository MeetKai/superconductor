mod lib;

#[tokio::main]
async fn main() {
    env_logger::init();
    pollster::block_on(lib::run());
}
