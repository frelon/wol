use std::error::Error;
use std::net::{SocketAddr, Ipv4Addr};

use clap::Parser;
use eui48::MacAddress;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    mac: MacAddress,

    #[clap(long, value_parser)]
    destination: Option<SocketAddr>,

    #[clap(long, value_parser)]
    source: Option<SocketAddr>,

    #[clap(short, long, value_parser)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mac_address = args.mac.as_bytes();

    let magic_packet = wake_on_lan::MagicPacket::new(mac_address.try_into().expect("must be a [u8;6]"));

    let dest = args.destination.unwrap_or_else(|| (Ipv4Addr::new(255, 255, 255, 255), 9).try_into().expect("is an addr"));
    let src = args.source.unwrap_or_else(|| (Ipv4Addr::new(0,0,0,0), 0).try_into().expect("is an addr"));

    if args.debug {
        println!("sending WOL packet for MAC {}, src {}, dest {}", args.mac, src, dest);
    }

    magic_packet.send_to(dest, src)?;

    if args.debug {
        println!("packet sent.");
    }

    Ok(())
}
