// Based on https://github.com/tokio-rs/tokio/blob/master/examples/proxy.rs
//
// Copyright (c) 2022 Tokio Contributors and Marlin Contributors
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use futures::FutureExt;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_vsock::{VsockAddr, VsockStream};

use oyster_tcp_proxy::addr_info::AddrInfo;

/// Creates a ip proxy for vsock server.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// ip address of the proxy to be set up <ip:port>
    #[clap(short, long, value_parser)]
    ip_addr: String,
    /// vsock address of the listener <cid>
    #[clap(short, long, value_parser)]
    vsock: u32,
}

#[tokio::main]
pub async fn port_to_vsock(ip_addr: &String, cid: u32) -> Result<()> {
    let listen_addr = ip_addr;

    println!("Listening on: {}", listen_addr);
    println!("Proxying to: {:?}", cid);

    let listener = TcpListener::bind(listen_addr)
        .await
        .context("failed to bind listener")?;

    while let Ok((inbound, _)) = listener.accept().await {
        let transfer = transfer(inbound, cid).map(|r| {
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(transfer);
    }

    Ok(())
}

async fn transfer(mut inbound: TcpStream, cid: u32) -> Result<()> {
    let inbound_addr = inbound
        .peer_addr()
        .context("could not fetch inbound addr")?
        .to_string();

    let orig_dst = inbound
        .get_original_dst()
        .ok_or(anyhow!("Failed to retrieve original destination"))?;
    println!("Original destination: {}", orig_dst);

    let proxy_addr = VsockAddr::new(cid, orig_dst.port().into());

    let outbound = VsockStream::connect(proxy_addr)
        .await
        .context("failed to connect vsock")?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = io::split(outbound);

    let client_to_server = async {
        io::copy(&mut ri, &mut wo)
            .await
            .context("error in port to vsock copy")
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi)
            .await
            .context("error in vsock to port copy")
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client).with_context(|| {
        format!(
            "error in connection between {} and {}",
            inbound_addr, proxy_addr
        )
    })?;

    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let x = port_to_vsock(&cli.ip_addr, cli.vsock);
    println!("{:?}", x);
}
