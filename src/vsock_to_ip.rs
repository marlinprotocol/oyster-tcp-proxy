//https://github.com/tokio-rs/tokio/blob/master/examples/proxy.rs


#![warn(rust_2018_idioms)]

use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_vsock::{VsockListener, VsockStream};
use futures::FutureExt;
use clap::Parser;

use std::env;
use std::error::Error;


#[path = "utils.rs"] mod utils;


/// Creates a vsock proxy for ip server.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// vsock address of the proxy to be set up <cid:port>
    #[clap(short, long, value_parser)]
    vsock_addr: String,
    /// ip address of the listener <ip:port>
    #[clap(short, long, value_parser)]
    ip_addr: String,
}

#[tokio::main]
pub async fn vsock_to_ip(cid: u32, port: u32, ip_addr: &String) -> Result<(), Box<dyn Error>> {
    let listen_addr = (cid, port);
    let server_addr = ip_addr;

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

fn main(){
    let cli = Cli::parse();
    let x = utils::split_vsock(&cli.vsock_addr).expect("vsock address not valid");
    match x {
        Some((cid, port)) => {
            let x = vsock_to_ip(cid, port, &cli.ip_addr);
            println!("{:?}", x);
        },
        None => {}
    }
}