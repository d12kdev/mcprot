use crate::types::packet::{Packet, ServerPacket};


#[derive(Debug)]
pub struct StatusRequest {
    _nothing: bool // EMPTY
}

impl Packet for StatusRequest {
    const PACKET_ID: i32 = 0x00;
}

impl ServerPacket for StatusRequest {
    fn read(_bytebuffer: &mut crate::types::ByteBuffer) -> color_eyre::eyre::Result<Self> {
        Ok(
            Self {
                _nothing: true
            }
        )
    }
}