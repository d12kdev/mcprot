use crate::types::{packet::{ClientPacket, Packet}, ByteBuffer, VarInt};


#[derive(Debug)]
pub struct EncryptionRequest {
    pub server_id: String, // String(20) Note: Appeares to be empty
    pub public_key_length: VarInt,
    pub public_key: ByteBuffer,
    pub verify_token_length: VarInt,
    pub verify_token: ByteBuffer,
    pub should_authenticate: bool
}

impl Packet for EncryptionRequest {
    const PACKET_ID: i32 = 0x01;
}

impl EncryptionRequest {
    pub fn new(
        server_id: String,
        public_key: ByteBuffer,
        verify_token: ByteBuffer,
        should_authenticate: bool
    ) -> Self {
        let public_key_length = public_key.len() as i32;
        let verify_token_length = verify_token.len() as i32;

        Self {
            server_id,
            public_key_length: VarInt::new(public_key_length),
            public_key,
            verify_token_length: VarInt::new(verify_token_length),
            verify_token,
            should_authenticate
        }
    }
}

impl ClientPacket for EncryptionRequest {
    fn get_payload(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_string(self.server_id.clone()).unwrap();
        buffer.put_varint(self.public_key_length);
        buffer.put_slice(self.public_key.to_u8());
        buffer.put_varint(self.verify_token_length);
        buffer.put_slice(self.verify_token.to_u8());
        buffer.put_bool(self.should_authenticate);

        buffer
    }
}