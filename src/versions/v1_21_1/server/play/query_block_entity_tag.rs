use crate::types::{packet::{Packet, ServerPacket}, BlockLocation, VarInt};


#[derive(Debug)]
pub struct QueryBlockEntityTag {
    pub transaction_id: VarInt,
    pub location: BlockLocation
}

impl Packet for QueryBlockEntityTag {
    const PACKET_ID: i32 = 0x01;
}

impl ServerPacket for QueryBlockEntityTag {
    fn decode(bytebuffer: &mut crate::types::ByteBuffer) -> color_eyre::eyre::Result<Self> {
        Ok(
            Self {
                transaction_id: bytebuffer.get_varint().unwrap(),
                location: bytebuffer.get_block_location()
            }
        )
    }
}