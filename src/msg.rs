use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_with::skip_serializing_none;
use tokio_nflog::{AddressFamily, L3Protocol, Message};

use super::hwheader::HwHeader;
use super::packet::Packet;

#[skip_serializing_none]
#[derive(Serialize)]
pub struct NflogMessage {
    #[serde(rename = "oob.family")]
    #[serde(serialize_with = "ser::serialize_af")]
    family: Option<AddressFamily>,
    #[serde(rename = "oob.protocol")]
    protocol: L3Protocol,
    #[serde(rename = "oob.prefix")]
    prefix: String,
    #[serde(rename = "oob.mark")]
    nfmark: u32,
    timestamp: DateTime<Utc>,
    #[serde(rename = "oob.ifindex_in")]
    indev: u32,
    #[serde(rename = "oob.ifindex_out")]
    outdev: u32,
    #[serde(rename = "oob.uid")]
    uid: Option<u32>,
    #[serde(rename = "oob.gid")]
    gid: Option<u32>,
    #[serde(rename = "oob.seq.local")]
    local_seqnum: Option<u32>,
    #[serde(rename = "oob.seq.global")]
    global_seqnum: Option<u32>,

    #[serde(flatten)]
    hw_header: HwHeader,

    #[serde(flatten)]
    packet: Option<Packet>,
}

mod ser {
    use super::AddressFamily;
    use serde::{self, Serializer};

    pub fn serialize_af<S>(family: &Option<AddressFamily>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match family.as_ref() {
            Some(&family) => serializer.serialize_i32(family as i32),
            None => serializer.serialize_none(),
        }
    }
}

impl NflogMessage {
    pub(crate) fn new(msg: Message<'_>) -> Self {
        let hw_header = HwHeader::new(&msg);
        let packet = Packet::new(&msg);

        Self {
            family: msg.address_family(),
            protocol: hw_header.protocol,
            prefix: msg.prefix().to_string(),
            nfmark: msg.nfmark(),
            timestamp: msg.timestamp().map(Into::into).unwrap_or_else(Utc::now),
            indev: msg.indev(),
            outdev: msg.outdev(),
            uid: msg.uid(),
            gid: msg.gid(),
            local_seqnum: msg.local_seqnum(),
            global_seqnum: msg.global_seqnum(),

            hw_header,
            packet,
        }
    }
}
