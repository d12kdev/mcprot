use crate::{common::{packet::encode_packet, text::TextComponent}, types::{packet::{ClientPacket, Packet}, ByteBuffer}};


#[derive(Debug)]
pub struct LoginDisconnect {
    reason: TextComponent
}

impl Packet for LoginDisconnect {
    const PACKET_ID: i32 = 0x00;
}

impl LoginDisconnect {
    pub fn new(reason: TextComponent) -> Self {
        Self { reason: reason }
    }
}

impl ClientPacket for LoginDisconnect {

    fn write(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_textcomponent(self.reason.clone());
        encode_packet(Self::PACKET_ID, buffer)
    }
}