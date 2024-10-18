use crate::types::packet::{Packet, ServerPacket};


#[derive(Debug)]
pub struct AcknowledgeFinishConfiguration;

impl Packet for AcknowledgeFinishConfiguration {
    const PACKET_ID: i32 = 0x03;
}

impl ServerPacket for AcknowledgeFinishConfiguration {
    fn read(_bytebuffer: &mut crate::types::ByteBuffer) -> color_eyre::eyre::Result<Self> {
        Ok(Self)
    }
}