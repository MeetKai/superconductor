mod lib;

fn main() {
    env_logger::init();
    async_std::task::block_on(lib::run());
}
