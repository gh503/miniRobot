use clap::{Arg, Command};

include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn main() {
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(ABOUT)
        .after_help(COPYRIGHT)
        .arg(
            Arg::new("peer")
                .short('p')
                .long("peer")
                .value_name("PEER_ADDRESS")
                .default_value("127.0.0.1:50000")
                .help("Sets the peer address")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("net-addr")
                .short('n')
                .long("net-addr")
                .value_name("NET_ADDRESS")
                .help("Sets the network address (IP or domain name)")
                .default_value("1.1.1.1")
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    let peer = if let Some(peer) = matches.get_one::<String>("peer") {
        Some(peer)
    } else {
        None
    };

    let net_addr = if let Some(net_addr) = matches.get_one::<String>("net-addr") {
        Some(net_addr)
    } else {
        None
    };

    let peer = peer.unwrap();
    let net_addr = net_addr.unwrap();

    println!("input peers: {}", &peer);
    println!("input network_address: {}", &net_addr);

}