use clap::{Parser, Subcommand};


mod vsock_to_ip;
mod ip_to_vsock;
mod utils;


/// Creates a vsock proxy for ip server and Viz.
 #[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
   /// configuration of the proxy
   #[clap(subcommand)]
   config: Option<Configs>,

}

#[derive(Subcommand)]
enum Configs {
    /// Creates a new ip proxy for a vsock listener
   Vsock_to_ip {
      /// vsock address of the listener <cid:port>
      #[clap(short, long, value_parser)]
      vsock_addr: String,
      /// ip address of the proxy to be set up <ip:port>
      #[clap(short, long, value_parser)]
      ip_addr: String,
   },
   /// Creates a new vsock proxy for a ip listener
   Ip_to_vsock {
   /// ip address of the listener <ip:port>
   #[clap(short, long, value_parser)]
   ip_addr: String,
   /// vsock address of the proxy to be set up <cid:port>
   #[clap(short, long, value_parser)]
   vsock_addr: String,
   },
}


fn main() {
   let cli = Cli::parse();

   match &cli.config {
      Some(Configs::Vsock_to_ip { vsock_addr, ip_addr}) => {
        let x = utils::split_vsock(vsock_addr).expect("vsock address not valid");
        match x {
            Some((cid, port)) => {
                let x = vsock_to_ip::vsock_to_ip(cid, port, ip_addr);
                println!("{:?}", x);
            },
            None => {}
        }
      },
      Some(Configs::Ip_to_vsock { ip_addr, vsock_addr}) => {
        let x = utils::split_vsock(vsock_addr).expect("vsock address not valid");
        match x {
            Some((cid, port)) => {
                let x = ip_to_vsock::ip_to_vsock(ip_addr, cid, port);
                println!("{:?}", x);
            },
            None => {}
        }
      },
      None => {}
  }

}