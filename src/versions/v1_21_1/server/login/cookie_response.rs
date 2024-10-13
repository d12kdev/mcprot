use crate::types::{packet::{Packet, ServerPacket}, ByteBuffer, VarInt};


#[derive(Debug)]
pub struct CookieResponse {
    pub key: String,
    pub has_payload: bool,
    pub payload_length: Option<VarInt>,
    pub payload: Option<ByteBuffer>
}


impl Packet for CookieResponse {
    const PACKET_ID: i32 = 0x04;
}

impl ServerPacket for CookieResponse {
    fn read(bytebuffer: &mut ByteBuffer) -> color_eyre::eyre::Result<Self> {
        let key = bytebuffer.get_string_maxsize(32767).unwrap();
        let has_payload = bytebuffer.get_bool();

        if has_payload {
            let payload_length = bytebuffer.get_varint().unwrap();
            let payload = bytebuffer.get_slice();

            Ok(
                Self {
                    key,
                    has_payload,
                    payload_length: Option::from(payload_length),
                    payload: Option::from(payload)
                }
            )
        } else {
            Ok(
                Self {
                    key,
                    has_payload,
                    payload_length: None,
                    payload: None
                }
            )
        }
    }
}