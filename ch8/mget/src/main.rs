use clap::{Arg, Command};
use smoltcp::phy::TapInterface;

use url::Url;

fn main() {
  let app = Command::new("mget")
    .about("GET a webpage, manually")
    .arg(Arg::new("url").required(true))
    .arg(Arg::new("tap-device").required(true))
    .arg(Arg::new("dns-server").default_value("1.1.1.1"))
    .get_matches();

  let url_text = app.get_one::<String>("url").unwrap();
  let dns_server_text = app.get_one::<String>("dns-server").unwrap();
  let tap_text = app.get_one::<String>("tap-device").unwrap();

  let url = Url::parse(url_text).expect("error: unable to parse <url> as a URL");

  if url.scheme() != "http" {
    eprintln!("error: only HTTP protocol supported");
    return;
  }

  let tap = TapInterface::new(&tap_text).expect("error: unable to use <tap-device> as a network interface");

  let domain_name = url.host_str().expect("domain name required");

  let _dns_server: std::net::Ipv4Addr = dns_server_text
    .parse()
    .expect("error: unable to parse <dns-server> as an IPv4 address");

  let addr = dns::resolve(dns_server_text, domain_name).unwrap().unwrap();

  let mac = ethernet::MacAddress::new().into();

  http::get(tap, mac, addr, url).unwrap();
}
