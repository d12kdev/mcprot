use crate::{common::{packet::encode_packet, Identifier}, types::{packet::{ClientPacket, Packet}, ByteBuffer, VarInt}};


#[derive(Debug)]
pub struct StoreCookie {
    key: Identifier,
    payload_length: VarInt,
    payload: ByteBuffer
}

impl Packet for StoreCookie {
    const PACKET_ID: i32 = 0x0A;
}

impl StoreCookie {
    pub fn new(key: Identifier, payload: ByteBuffer) -> Self {
        let payload_length = VarInt::from(payload.len() as i32);

        Self {
            key,
            payload_length,
            payload
        }
    }
}

impl ClientPacket for StoreCookie {
    fn write(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_identifier(self.key.clone());
        buffer.put_varint(self.payload_length);
        buffer.put_slice(self.payload.to_u8());

        encode_packet(Self::PACKET_ID, buffer)
    }
}