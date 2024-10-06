use crate::{common::packet::encode_packet, types::{packet::{ClientPacket, Packet}, ByteBuffer}};

#[derive(Debug)]
pub struct PongResponse {
    payload: i64
}

impl Packet for PongResponse {
    const PACKET_ID: i32 = 0x01;
}

impl PongResponse {
    pub fn new(payload: i64) -> Self {
        Self { payload: payload }
    }
}

impl ClientPacket for PongResponse {
    fn write(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_i64(self.payload);
        encode_packet(Self::PACKET_ID, buffer)
    }
}