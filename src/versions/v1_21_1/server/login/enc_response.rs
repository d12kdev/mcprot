use crate::types::{packet::{Packet, ServerPacket}, VarInt};


#[derive(Debug)]
pub struct EncryptionResponse {
    pub shared_secret_length: VarInt,
    pub shared_secret: Vec<u8>,
    pub verify_token_lenght: VarInt,
    pub verify_token: Vec<u8>
}

impl Packet for EncryptionResponse {
    const PACKET_ID: i32 = 0x01;
}

impl ServerPacket for EncryptionResponse {
    fn read(bytebuffer: &mut crate::types::ByteBuffer) -> color_eyre::eyre::Result<Self> {
        let shared_secret_length = bytebuffer.get_varint().unwrap();
        let shared_secret = bytebuffer.copy_to_bytes(shared_secret_length.get_value() as usize).unwrap();
        let verify_token_lenght = bytebuffer.get_varint().unwrap();
        let verify_token = bytebuffer.copy_to_bytes(verify_token_lenght.get_value() as usize).unwrap();

        Ok(
            Self {
                shared_secret_length,
                shared_secret: shared_secret.to_vec(),
                verify_token_lenght,
                verify_token: verify_token.to_vec()
            }
        )
    }
}