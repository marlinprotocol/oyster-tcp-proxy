//https://github.com/tokio-rs/tokio/blob/master/examples/proxy.rs


#![warn(rust_2018_idioms)]

use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_vsock::VsockStream;
use futures::FutureExt;
use clap::Parser;

use std::env;
use std::error::Error;


#[path = "utils.rs"] mod utils;


/// Creates a ip proxy for vsock server.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// ip address of the proxy to be set up <ip:port>
    #[clap(short, long, value_parser)]
    ip_addr: String,
    /// vsock address of the listener <cid:port>
    #[clap(short, long, value_parser)]
    vsock_addr: String,
}

#[tokio::main]
pub async fn ip_to_vsock(ip_addr: &String, cid: u32, port: u32) -> Result<(), Box<dyn Error>> {
    let listen_addr = ip_addr;
    let server_addr = (cid, port);

    println!("Listening on: {}", listen_addr);
    println!("Proxying to: {:?}", server_addr);

    let listener = TcpListener::bind(listen_addr).await?;

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

async fn transfer(mut inbound: TcpStream, proxy_addr: (u32, u32)) -> Result<(), Box<dyn Error>> {
    let mut outbound = VsockStream::connect(proxy_addr.0, proxy_addr.1).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = io::split(outbound);

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
            let x = ip_to_vsock(&cli.ip_addr, cid, port);
            println!("{:?}", x);
        },
        None => {}
    }
}