use pnet_packet::ipv4::Ipv4Packet;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::net::Ipv4Addr;
use tokio_nflog::{AddressFamily, Message};

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Packet {
    Ipv4(Ipv4),
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct Ipv4 {
    #[serde(rename = "ip.saddr")]
    pub saddr: Ipv4Addr,
    #[serde(rename = "ip.daddr")]
    pub daddr: Ipv4Addr,
    #[serde(rename = "ip.protocol")]
    pub protocol: u8,
    #[serde(rename = "ip.tos")]
    pub tos: u8,
    #[serde(rename = "ip.ttl")]
    pub ttl: u8,
    #[serde(rename = "ip.totlen")]
    pub total_len: u16,
    #[serde(rename = "ip.ihl")]
    pub header_len: u8,
    #[serde(rename = "ip.csum")]
    pub checksum: u16,
    #[serde(rename = "ip.id")]
    pub id: u16,
    #[serde(rename = "ip.fragoff")]
    pub fragoff: u16,
}

impl Packet {
    pub fn new(msg: &Message<'_>) -> Option<Self> {
        let family = msg.address_family()?;
        let payload = msg.payload()?;

        match family {
            AddressFamily::Inet => Some(Packet::Ipv4(Self::parse_inet(payload)?)),
            // AddressFamily::Inet6 => Self::parse_inet6(msg, payload),
            // AddressFamily::Bridge => Self::parse_bridge(msg, payload),
            _ => None,
        }
    }

    fn parse_inet(payload: &[u8]) -> Option<Ipv4> {
        let packet = Ipv4Packet::new(payload)?;
        let protocol = packet.get_next_level_protocol();

        Some(Ipv4 {
            saddr: packet.get_source(),
            daddr: packet.get_destination(),
            protocol: protocol.0,
            tos: packet.get_dscp() as u8,
            ttl: packet.get_ttl(),
            total_len: packet.get_total_length(),
            header_len: packet.get_header_length(),
            checksum: packet.get_checksum(),
            id: packet.get_identification(),
            fragoff: packet.get_fragment_offset(),
        })
    }

    // fn parse_inet6(msg: &Message<'_>, payload: &[u8]) {}

    // fn parse_bridge(msg: &Message<'_>, payload: &[u8]) {}
}
