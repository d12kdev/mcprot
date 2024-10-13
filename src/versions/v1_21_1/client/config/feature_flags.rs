use crate::{common::{packet::encode_packet, Identifier}, types::{packet::{ClientPacket, Packet}, ByteBuffer, VarInt}};


#[derive(Debug)]
pub struct FeatureFlags {
    total_features: VarInt,
    feature_flags: Identifier
}

impl Packet for FeatureFlags {
    const PACKET_ID: i32 = 0x0C;
}

impl FeatureFlags {
    pub fn new(total_features: VarInt, feature_flags: Identifier) -> Self {
        Self {
            total_features,
            feature_flags
        }
    }
}

impl ClientPacket for FeatureFlags {
    fn write(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_varint(self.total_features);
        buffer.put_identifier(self.feature_flags.clone());

        encode_packet(Self::PACKET_ID, buffer)
    }
}