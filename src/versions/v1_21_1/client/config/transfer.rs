use crate::types::{packet::{ClientPacket, Packet}, ByteBuffer, VarInt};


#[derive(Debug)]
pub struct Transfer {
    host: String,
    port: VarInt
}

impl Packet for Transfer {
    const PACKET_ID: i32 = 0x0B;
}

impl Transfer {
    pub fn new(host: String, port: VarInt) -> Self {
        Self { host: host, port: port }
    }
}

impl ClientPacket for Transfer {
    fn get_payload(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_string(self.host.clone()).unwrap();
        buffer.put_varint(self.port);

        buffer
    }
}