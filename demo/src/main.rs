mod lib;

#[tokio::main]
async fn main() {
    pollster::block_on(lib::run());
}
