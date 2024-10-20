use crate::types::{packet::{ClientPacket, Packet}, ByteBuffer, Location, VarInt};


#[derive(Debug)]
pub struct SpawnExperienceOrb {
    pub entity_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub count: i16
}

impl Packet for SpawnExperienceOrb {
    const PACKET_ID: i32 = 0x02;
}

impl SpawnExperienceOrb {
    pub fn new(entity_id: VarInt, location: Location, count: i16) -> Self {
        Self {
            entity_id,
            x: location.get_x(),
            y: location.get_y(),
            z: location.get_z(),
            count
        }
    }
}

impl ClientPacket for SpawnExperienceOrb {
    fn get_payload(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_varint(self.entity_id);

        buffer.put_f64(self.x);
        buffer.put_f64(self.y);
        buffer.put_f64(self.z);

        buffer.put_i16(self.count);

        buffer
    }
}