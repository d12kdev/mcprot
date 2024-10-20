use crate::types::{packet::{Packet, ServerPacket}, VarInt};


#[derive(Debug)]
pub struct ConfirmTeleportation {
    pub teleport_id: VarInt
}

impl Packet for ConfirmTeleportation {
    const PACKET_ID: i32 = 0x00;
}

impl ServerPacket for ConfirmTeleportation {
    fn decode(bytebuffer: &mut crate::types::ByteBuffer) -> color_eyre::eyre::Result<Self> {
        Ok(
            Self {
                teleport_id: bytebuffer.get_varint().unwrap()
            }
        )
    }
}