use std::{env, process::exit};

use easyproxy::proxy_tcp;

mod macros;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    if args.len() < 3 {
        fatal!("Example usage: easyproxy 0.0.0.0:2345 127.0.0.1:4567");
    }
    proxy_tcp(&args[1], &args[2]).await
}
