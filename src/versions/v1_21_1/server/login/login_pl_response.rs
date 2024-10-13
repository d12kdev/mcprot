use crate::types::{packet::{Packet, ServerPacket}, ByteBuffer, VarInt};


#[derive(Debug)]
pub struct LoginPluginResponse {
    pub message_id: VarInt,
    pub successful: bool,
    pub data: Option<ByteBuffer>
}

impl Packet for LoginPluginResponse {
    const PACKET_ID: i32 = 0x02;
}

impl ServerPacket for LoginPluginResponse {
    fn read(bytebuffer: &mut ByteBuffer) -> color_eyre::eyre::Result<Self> {
        Ok(
            Self {
                message_id: bytebuffer.get_varint().unwrap(),
                successful: bytebuffer.get_bool(),
                data: bytebuffer.get_option(|v| Ok(v.get_slice())).unwrap()
            }
        )
    }
}