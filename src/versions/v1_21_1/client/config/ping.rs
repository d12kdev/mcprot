use crate::types::{packet::{ClientPacket, Packet}, ByteBuffer};


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
    fn get_payload(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_i32(self.id);

        buffer
    }
}