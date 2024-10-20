use uuid::Uuid;

use crate::{common::ResourcePackResponseResult, types::packet::{Packet, ServerPacket}};


#[derive(Debug)]
pub struct ResourcePackResponse {
    pub uuid: Uuid,
    pub result: ResourcePackResponseResult
}

impl Packet for ResourcePackResponse {
    const PACKET_ID: i32 = 0x06;
}

impl ServerPacket for ResourcePackResponse {
    fn decode(bytebuffer: &mut crate::types::ByteBuffer) -> color_eyre::eyre::Result<Self> {
        Ok(
            Self {
                uuid: bytebuffer.get_uuid()?,
                result: ResourcePackResponseResult::from(bytebuffer.get_varint()?)
            }
        )
    }
}