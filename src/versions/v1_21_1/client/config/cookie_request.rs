use crate::{common::{packet::encode_packet, Identifier}, types::{packet::{ClientPacket, Packet}, ByteBuffer}};


#[derive(Debug)]
pub struct CookieRequest {
    key: Identifier
}

impl Packet for CookieRequest {
    const PACKET_ID: i32 = 0x00;
}

impl CookieRequest {
    pub fn new(key: Identifier) -> Self {
        Self { key: key }
    }
}

impl ClientPacket for CookieRequest {
    fn write(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_identifier(self.key.clone());
        
        encode_packet(Self::PACKET_ID, buffer)
    }
}