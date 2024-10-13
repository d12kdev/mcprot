use uuid::Uuid;

use crate::{common::packet::encode_packet, types::{packet::{ClientPacket, Packet}, ByteBuffer}};


#[derive(Debug)]
pub struct RemoveResourcepack {
    has_uuid: bool,
    uuid: Option<Uuid>
}

impl Packet for RemoveResourcepack {
    const PACKET_ID: i32 = 0x08;
}

impl RemoveResourcepack {
    pub fn new(uuid: Option<Uuid>) -> Self {
        let has_uuid = !uuid.is_none();

        Self { has_uuid: has_uuid, uuid: uuid }
    }
}

impl ClientPacket for RemoveResourcepack {
    fn write(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_bool(self.has_uuid);

        if self.has_uuid {
            buffer.put_uuid(self.uuid.clone().unwrap());
        }

        encode_packet(Self::PACKET_ID, buffer)
    }
}