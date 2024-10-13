use crate::types::packet::{Packet, ServerPacket};


#[derive(Debug)]
pub struct LoginAcknowledged;

impl Packet for LoginAcknowledged {
    const PACKET_ID: i32 = 0x03;
}

impl ServerPacket for LoginAcknowledged {
    fn read(_bytebuffer: &mut crate::types::ByteBuffer) -> color_eyre::eyre::Result<Self> {
        Ok(
            Self
        )
    }
}