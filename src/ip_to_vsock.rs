#![warn(rust_2018_idioms)]

use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use tokio_vsock::{VsockListener, VsockStream};

use futures::FutureExt;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listen_addr = (16, 8001);
    let server_addr = env::args()
    .nth(1)
    .unwrap_or_else(|| "127.0.0.1:80".to_string());

    println!("Listening on: {:?}", listen_addr);
    println!("Proxying to: {:?}", server_addr);

    let mut listener = VsockListener::bind(listen_addr.0, listen_addr.1).
        expect("listener failed");

    while let Ok((inbound, _)) = listener.accept().await {
        let transfer = transfer(inbound, server_addr.clone()).map(|r| { 
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(transfer);
    }

    Ok(())
}

async fn transfer(mut inbound: VsockStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = io::split(inbound);
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}
