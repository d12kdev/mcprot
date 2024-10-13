use crate::{common::packet::encode_packet, protocol::Animation, types::{packet::{ClientPacket, Packet}, ByteBuffer, VarInt}};


#[derive(Debug)]
pub struct EntityAnimation {
    entity_id: VarInt,
    animation: Animation
}

impl Packet for EntityAnimation {
    const PACKET_ID: i32 = 0x03;
}

impl EntityAnimation {
    pub fn new(entity_id: VarInt, animation: Animation) -> Self {
        Self {
            entity_id: entity_id,
            animation: animation
        }
    }
}

impl ClientPacket for EntityAnimation {
    fn write(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_varint(self.entity_id);
        buffer.put_u8(self.animation.into());

        encode_packet(Self::PACKET_ID, buffer)
    }
}