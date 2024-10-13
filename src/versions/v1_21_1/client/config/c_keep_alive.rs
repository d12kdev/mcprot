use crate::{common::packet::encode_packet, types::{packet::{ClientPacket, Packet}, ByteBuffer}};


#[derive(Debug)]
pub struct ClientboundKeepAlive {
    keep_alive_id: i64
}

impl Packet for ClientboundKeepAlive {
    const PACKET_ID: i32 = 0x04;
}

impl ClientboundKeepAlive {
    pub fn new() -> Self {
        Self { keep_alive_id: rand::random::<i64>() }
    }
}

impl ClientPacket for ClientboundKeepAlive {
    fn write(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_i64(self.keep_alive_id);

        encode_packet(Self::PACKET_ID, buffer)
    }
}