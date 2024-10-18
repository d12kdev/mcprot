use crate::types::packet::{Packet, ServerPacket};


#[derive(Debug)]
pub struct ServerboundKeepAlive {
    pub keep_alive_id: i64
}

impl Packet for ServerboundKeepAlive {
    const PACKET_ID: i32 = 0x04;
}

impl ServerPacket for ServerboundKeepAlive {
    fn read(bytebuffer: &mut crate::types::ByteBuffer) -> color_eyre::eyre::Result<Self> {
        Ok(
            Self {
                keep_alive_id: bytebuffer.get_i64()
            }
        )
    }
}