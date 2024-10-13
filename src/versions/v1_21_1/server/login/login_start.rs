use uuid::Uuid;

use crate::types::packet::{Packet, ServerPacket};


#[derive(Debug)]
pub struct LoginStart {
    pub name: String, // String(16)
    pub player_uuid: Uuid
}

impl Packet for LoginStart {
    const PACKET_ID: i32 = 0x00;
}

impl ServerPacket for LoginStart {
    fn read(bytebuffer: &mut crate::types::ByteBuffer) -> color_eyre::eyre::Result<Self> {
        Ok(
            Self {
                name: bytebuffer.get_string_maxsize(16).unwrap(),
                player_uuid: bytebuffer.get_uuid().unwrap()
            }
        )
    }
}