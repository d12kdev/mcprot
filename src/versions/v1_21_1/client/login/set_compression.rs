use crate::{common::packet::encode_packet, types::{packet::{ClientPacket, Packet}, ByteBuffer, VarInt}};


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
    fn write(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_varint(self.threshold);
        encode_packet(Self::PACKET_ID, buffer)
    }
}