use crate::types::{packet::{ClientPacket, Packet}, ByteBuffer, VarInt};


#[derive(Debug)]
pub struct SetCompression {
    threshold: VarInt
}

impl Packet for SetCompression {
    const PACKET_ID: i32 = 0x03;
}

impl SetCompression {
    pub fn new(threshold: VarInt) -> Self {
        Self { threshold: threshold }
    }
}

impl ClientPacket for SetCompression {
    fn get_payload(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_varint(self.threshold);
        buffer
    }
}