use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::convert::TryInto;
use std::fmt::Write;
use tokio_nflog::{MacAddr, Message};

#[skip_serializing_none]
#[derive(Serialize)]
pub struct HwHeader {
    #[serde(skip_serializing)]
    pub protocol: u16,
    #[serde(rename = "raw.type")]
    pub hwtype: Option<u16>,
    #[serde(rename = "mac.saddr.str")]
    pub saddr: Option<MacAddr>,
    #[serde(rename = "mac.daddr.str")]
    pub daddr: Option<MacAddr>,
    #[serde(rename = "mac.str")]
    pub mac_str: Option<String>,
}

fn parse_as_mac(raw: &[u8]) -> Option<MacAddr> {
    let mac: [u8; 6] = raw
        .try_into()
        .map_err(|e| println!("MAC error: {}", e))
        .ok()?;

    Some(mac.into())
}

fn mac2str(raw: &[u8]) -> String {
    let len = raw.len();
    let size = (len << 1) + (len - 1);
    raw.iter()
        .fold(String::with_capacity(size), |mut acc, &item| {
            if !acc.is_empty() {
                acc.push(':');
            }

            write!(acc, "{:02x}", item).expect("Cannot convert mac to str");
            acc
        })
}

impl HwHeader {
    fn default(protocol: u16) -> Self {
        Self {
            protocol,
            hwtype: None,
            saddr: None,
            daddr: None,
            mac_str: None,
        }
    }

    pub(crate) fn new(msg: &Message<'_>) -> Self {
        let mut hw_hdr = Self::default(msg.l3_proto());

        if let Some(saddr) = msg.packet_hwaddr() {
            hw_hdr.saddr = Some(saddr);
            hw_hdr.hwtype = Some(libc::ARPHRD_VOID);
        }

        if let Some(hwhdr) = msg.packet_hwhdr() {
            let hwtype = msg.hwtype();
            println!("{}", hwhdr.len());
            match hwtype {
                libc::ARPHRD_ETHER if hwhdr.len() >= 14 => {
                    hw_hdr.saddr = parse_as_mac(&hwhdr[0..6]);
                    hw_hdr.daddr = parse_as_mac(&hwhdr[6..12]);
                    if let Ok(hwhdr_proto) = (&hwhdr[12..14]).read_u16::<BigEndian>() {
                        hw_hdr.protocol = hwhdr_proto;
                    }
                }
                _ => hw_hdr.mac_str = Some(mac2str(hwhdr)),
            };
            hw_hdr.hwtype = Some(hwtype)
        }

        hw_hdr
    }
}
