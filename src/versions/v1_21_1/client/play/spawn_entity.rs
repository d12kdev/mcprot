use uuid::Uuid;

use crate::{common::packet::encode_packet, protocol::Entity, types::{packet::{ClientPacket, Packet}, Angle, ByteBuffer, Location, VarInt}};

#[derive(Debug)]
pub struct SpawnEntity {
    pub entity_id: VarInt,
    pub entity_uuid: Uuid,
    pub entity_type: Entity,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: Angle,
    pub yaw: Angle,
    pub head_yaw: Angle,
    pub data: VarInt,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16
}

impl Packet for SpawnEntity {
    const PACKET_ID: i32 = 0x01;
}

impl SpawnEntity {
    pub fn new(
        entity_id: VarInt,
        entity_uuid: Uuid,
        entity_type: Entity,
        location: Location,
        pitch: Angle,
        yaw: Angle,
        head_yaw: Angle,
        data: VarInt,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16
    ) -> Self {
        Self {
            entity_id,
            entity_uuid,
            entity_type,
            x: location.get_x(),
            y: location.get_y(),
            z: location.get_z(),
            pitch,
            yaw,
            head_yaw,
            data,
            velocity_x,
            velocity_y,
            velocity_z,
        }
    }
}

impl ClientPacket for SpawnEntity {
    fn write(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_varint(self.entity_id);
        buffer.put_uuid(self.entity_uuid);
        buffer.put_varint(self.entity_type.to_varint());

        buffer.put_f64(self.x);
        buffer.put_f64(self.y);
        buffer.put_f64(self.z);

        buffer.put_angle(self.pitch);
        buffer.put_angle(self.yaw);
        buffer.put_angle(self.head_yaw);

        buffer.put_varint(self.data);
        
        buffer.put_i16(self.velocity_x);
        buffer.put_i16(self.velocity_y);
        buffer.put_i16(self.velocity_z);

        encode_packet(Self::PACKET_ID, buffer)
    }
}