use crate::{common::Identifier, types::{packet::{ClientPacket, Packet}, ByteBuffer}};


#[derive(Debug)]
pub struct ClientboundPluginMessage {
    channel: Identifier,
    data: ByteBuffer
}

impl Packet for ClientboundPluginMessage {
    const PACKET_ID: i32 = 0x01;
}

impl ClientboundPluginMessage {
    pub fn new(channel: Identifier, data: ByteBuffer) -> Self {
        Self { channel: channel, data: data }
    }
}

impl ClientPacket for ClientboundPluginMessage {
    fn get_payload(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_identifier(self.channel.clone());
        buffer.put_slice(self.data.to_u8());

        buffer
    }
}