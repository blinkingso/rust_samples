extern crate smoltcp;
extern crate structopt;
use std::net::Ipv4Addr;

use url::Url;
mod dns;
mod ethernet;
mod http;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "mget",
    author = "andrew",
    about = "a tool to execute http request."
)]
struct MgetOpt {
    /// A URL to download data from.
    #[structopt(short, long)]
    url: String,
    /// A TAP networking device to connect with.
    #[structopt(short, long)]
    network_device: String,
    /// (Optional) Possible for the user to select which DNS server to use, Default: 1.1.1.1
    #[structopt(short, long, default_value = "1.1.1.1")]
    dns_server: String,
}

/// use {} format display.
impl std::fmt::Display for MgetOpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"MgetOpt: {{"url": {}, "network_device": {}, "dns_server": {}}}"#,
            self.url, self.network_device, self.dns_server
        )
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args = MgetOpt::from_args();
    println!("received args: {}", &args);
    let url = args.url;
    let dns_server = args.dns_server;
    let network_device = args.network_device;
    let url = Url::parse(&url).expect("Unable to parse <url> as a URL");
    if url.scheme() != "http" {
        eprintln!("error: only HTTP protocol supported");
        return;
    }
    // let tap = TapInterface
    let domain_name = url.host_str().expect("domain name required");
    let _dns_server = dns_server
        .parse::<Ipv4Addr>()
        .expect("error: Unable to parse <dns-server> as an Ipv4 address");
    let addr = dns::resolve(&dns_server, domain_name).unwrap().unwrap();
    let mac_address = [
        0x00, 0xe0 as u8, 0x4c as u8, 0x68 as u8, 0x01 as u8, 0x04 as u8,
    ];
    // wifi
    let mac_address = [0xa4, 0x5e, 0x60, 0xc3, 0xd0, 0x45];
    println!("mac_address is : {:?}", mac_address);
    // let mac = ethernet::MacAddress::new().into();
    let mac = ethernet::MacAddress::new_with_buf(mac_address).into();
    let result = http::get(&network_device, mac, addr.to_string(), url);
    match result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("full error is : {:?}", err);
        }
    }
}
