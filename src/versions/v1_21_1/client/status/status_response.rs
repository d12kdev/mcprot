use crate::{common::{packet::encode_packet, ServerStatus}, types::{packet::{ClientPacket, Packet}, ByteBuffer}};

#[derive(Debug)]
pub struct StatusResponse {
    pub server_status: ServerStatus
}

impl Packet for StatusResponse {
    const PACKET_ID: i32 = 0x00;
}

impl StatusResponse {
    pub fn new(server_status: ServerStatus) -> Self {
        Self { server_status: server_status }
    }
}

impl ClientPacket for StatusResponse {
    fn write(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_string(self.server_status.to_json()).unwrap();
        encode_packet(Self::PACKET_ID, buffer)
    }
}