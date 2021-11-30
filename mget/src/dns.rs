use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4},
    str::FromStr,
};

use trust_dns_client::{
    client::{Client, SyncClient},
    proto::error::ProtoError,
    rr::{DNSClass, Name, RecordType},
    udp::UdpClientConnection,
};
/// resolve Ipv4 address from specified dns server, for default is 1.1.1.1
pub fn resolve(
    dns_server: &str,
    domain_name: &str,
) -> std::result::Result<Option<IpAddr>, Box<dyn std::error::Error>> {
    let dns_server = String::from(dns_server);
    let dns_server_ipv4 = dns_server.parse::<Ipv4Addr>()?;
    let dns_server = SocketAddr::V4(SocketAddrV4::new(dns_server_ipv4, 53));
    let conn = UdpClientConnection::new(dns_server)?;
    let domain_name = Name::from_str(domain_name)?;
    let client = SyncClient::new(conn);
    let response = client.query(&domain_name, DNSClass::IN, RecordType::A)?;
    let answers = response.answers();
    // check and return
    for answer in answers {
        if let Some(ip) = answer.rdata().to_ip_addr() {
            return Ok(Some(ip));
        }
    }
    Ok(None)
}

#[derive(Debug)]
pub enum DnsError {
    ClientError(trust_dns_client::error::ClientError),
    ParseDomainName(ProtoError),
    ParseDnsServerAddress(std::net::AddrParseError),
}
impl std::fmt::Display for DnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self)
    }
}
impl std::error::Error for DnsError {}

impl From<std::net::AddrParseError> for DnsError {
    fn from(err: std::net::AddrParseError) -> Self {
        DnsError::ParseDnsServerAddress(err)
    }
}

impl From<trust_dns_client::error::ClientError> for DnsError {
    fn from(err: trust_dns_client::error::ClientError) -> Self {
        DnsError::ClientError(err)
    }
}

impl From<ProtoError> for DnsError {
    fn from(err: ProtoError) -> Self {
        DnsError::ParseDomainName(err)
    }
}

#[test]
fn test_dns_server() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _ = resolve("1.1.1.1", "baidu.com")?;
    Ok(())
}
