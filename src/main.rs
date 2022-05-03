use std::{env, process::exit};

use log::{error, info};
use tokio::net::{TcpListener, TcpStream};

mod macros;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    if args.len() < 3 {
        fatal!("Example usage: easyproxy 0.0.0.0:2345 127.0.0.1:4567");
    }
    let (local, forward) = (&args[1], &args[2]);
    let listener = TcpListener::bind(local).await?;

    info!("Proxy tcp packets from {} -> {}", local, forward);
    loop {
        let (mut local_stream, saddr) = listener.accept().await?;
        let forward = forward.clone();

        info!("New client from {}", saddr);

        tokio::spawn(async move {
            let upstream_ret = TcpStream::connect(forward.clone()).await;
            match upstream_ret {
                Ok(mut upstream) => {
                    let ret = tokio::io::copy_bidirectional(&mut local_stream, &mut upstream).await;
                    match ret {
                        Err(e) => info!("Disconnected, {}", e),
                        Ok((in_byte, out_byte)) => {
                            info!("Disconnected, in bytes={}, out bytes={}", in_byte, out_byte)
                        }
                    }
                }
                Err(e) => error!("Faild connect to {}, {}", forward, e),
            }
        });
    }
}
