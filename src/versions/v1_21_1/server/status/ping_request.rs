use crate::types::packet::{Packet, ServerPacket};

#[derive(Debug)]
pub struct PingRequest {
    pub payload: i64
}

impl Packet for PingRequest {
    const PACKET_ID: i32 = 0x01;
}

impl ServerPacket for PingRequest {
    fn read(bytebuffer: &mut crate::types::ByteBuffer) -> color_eyre::eyre::Result<Self> {
        Ok(
            Self {
                payload: bytebuffer.get_i64()
            }
        )
    }
}