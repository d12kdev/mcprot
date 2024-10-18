use crate::{common::Identifier, types::{packet::{Packet, ServerPacket}, ByteBuffer, VarInt}};


#[derive(Debug)]
pub struct CookieResponse {
    pub key: Identifier,
    pub has_payload: bool,
    pub payload_length: Option<VarInt>,
    pub payload: Option<ByteBuffer>
}

impl Packet for CookieResponse {
    const PACKET_ID: i32 = 0x01;
}

impl ServerPacket for CookieResponse {
    fn read(bytebuffer: &mut ByteBuffer) -> color_eyre::eyre::Result<Self> {
        let key = bytebuffer.get_identifier()?;
        let has_payload = bytebuffer.get_bool();
        let mut payload_length: Option<VarInt> = None;
        let mut payload: Option<ByteBuffer> = None;

        if has_payload {
            payload_length = Option::from(bytebuffer.get_varint()?);
            payload = Option::from(bytebuffer.get_slice());
        }

        Ok(
            Self {
                key,
                has_payload,
                payload_length,
                payload
            }
        )
    }
}