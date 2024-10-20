use crate::types::{packet::{ClientPacket, Packet}, ByteBuffer};


#[derive(Debug)]
pub struct ResetChat;

impl Packet for ResetChat {
    const PACKET_ID: i32 = 0x06;
}

impl ResetChat {
    pub fn new() -> Self {
        Self
    }
}

impl ClientPacket for ResetChat {
    fn get_payload(&self) -> crate::types::ByteBuffer {
        ByteBuffer::new()
    }
}