use crate::{common::Identifier, types::{packet::{Packet, ServerPacket}, ByteBuffer}};


#[derive(Debug)]
pub struct ServerPluginMessage {
    pub channel: Identifier,
    pub data: ByteBuffer
}

impl Packet for ServerPluginMessage {
    const PACKET_ID: i32 = 0x02;
}

impl ServerPacket for ServerPluginMessage {
    fn read(bytebuffer: &mut ByteBuffer) -> color_eyre::eyre::Result<Self> {
        Ok(
            Self { channel: bytebuffer.get_identifier()?, data: bytebuffer.get_slice() }
        )
    }
}