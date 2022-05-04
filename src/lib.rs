//! <div align="center">
//!   <h1>EasyProxy</h1>
//!   <h3>🐚 An async & dynamic 4 layer Tcp proxy for Rust</h3>
//!
//!   [![crate](https://img.shields.io/crates/v/easyproxy.svg)](https://crates.io/crates/easyproxy)
//!   [![docs](https://docs.rs/easyproxy/badge.svg)](https://docs.rs/easyproxy)
//!   [![build status](https://github.com/iwannay/easyproxy/actions/workflows/rust.yml/badge.svg)](https://github.com/iwannay/easyproxy/actions/workflows/rust.yml)
//! </div>
//!
//! # EasyProxy
//!
//! #### is a simple TCP forwarding tool used to help you connect different networks
//!
//! ## Getting Started
//!
//! ```sh
//! easyproxy localhost:2345 localhost:8080
//! ```
//!

use log::{error, info};
use tokio::net::{TcpListener, TcpStream};

/// Forward TCP traffic from local addr to upstream addr
pub async fn proxy_tcp(
    local: &String,
    upstream: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(local).await?;

    info!("Proxy tcp packets from {} -> {}", local, upstream);
    loop {
        let (mut local_stream, saddr) = listener.accept().await?;
        let forward = upstream.clone();

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
                Err(e) => error!("Failed connect to {}, {}", forward, e),
            }
        });
    }
}
