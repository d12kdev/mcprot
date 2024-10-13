use crate::{common::packet::encode_packet, types::{packet::{ClientPacket, Packet}, ByteBuffer}};


#[derive(Debug)]
pub struct ConfigPing {
    id: i32
}

impl Packet for ConfigPing {
    const PACKET_ID: i32 = 0x05;
}

impl ConfigPing {
    pub fn new(id: i32) -> Self {
        Self { id: id }
    }
}

impl ClientPacket for ConfigPing {
    fn write(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_i32(self.id);

        encode_packet(Self::PACKET_ID, buffer)
    }
}