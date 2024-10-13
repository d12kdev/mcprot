use crate::{common::packet::encode_packet, types::{packet::{ClientPacket, Packet}, ByteBuffer}};


#[derive(Debug)]
pub struct FinishConfig;

impl Packet for FinishConfig {
    const PACKET_ID: i32 = 0x03;
}

impl FinishConfig {
    pub fn new() -> Self {
        Self
    }
}

impl ClientPacket for FinishConfig {
    fn write(&self) -> crate::types::ByteBuffer {
        encode_packet(Self::PACKET_ID, ByteBuffer::new())
    }
}