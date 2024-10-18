use crate::types::packet::{Packet, ServerPacket};


#[derive(Debug)]
pub struct ConfigPong {
    pub id: i32
}

impl Packet for ConfigPong {
    const PACKET_ID: i32 = 0x05;
}

impl ServerPacket for ConfigPong {
    fn read(bytebuffer: &mut crate::types::ByteBuffer) -> color_eyre::eyre::Result<Self> {
        Ok(
            Self {
                id: bytebuffer.get_i32()
            }
        )
    }
}