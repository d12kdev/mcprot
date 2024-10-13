use crate::{common::{packet::encode_packet, Identifier}, types::{packet::{ClientPacket, Packet}, ByteBuffer, VarInt}};


#[derive(Debug)]
pub struct UpdateTags {
    tag_array_length: VarInt,
    tag_array: Vec<UpdateTagsRegistry>
}

#[derive(Debug)]
struct UpdateTagsRegistry {
    identifier: Identifier,
    tags: Vec<UpdateTagsTag>
}

impl UpdateTagsRegistry {
    pub fn encode(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_identifier(self.identifier.clone());

        for tag in &self.tags {
            buffer.put_slice(tag.encode().to_u8());
        }

        buffer
    }
}

#[derive(Debug)]
struct UpdateTagsTag {
    pub tag_name: Identifier,
    pub count: VarInt,
    pub entries: Vec<VarInt>, 
}

impl UpdateTagsTag {
    pub fn encode(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_identifier(self.tag_name.clone());
        buffer.put_varint(self.count);
        
        for entry in &self.entries {
            buffer.put_varint(*entry);
        }

        buffer
    }
}

impl Packet for UpdateTags {
    const PACKET_ID: i32 = 0x0D;
}

impl UpdateTags {
    #[allow(private_interfaces)]
    pub fn new_tag(name: Identifier, entries: Vec<VarInt>) -> UpdateTagsTag {
        UpdateTagsTag { tag_name: name, count: VarInt::from(entries.len() as i32), entries: entries }
    }

    #[allow(private_interfaces)]
    pub fn new_registry(identifier: Identifier, tags: Vec<UpdateTagsTag>) -> UpdateTagsRegistry {
        UpdateTagsRegistry {
            identifier,
            tags
        }
    }

    #[allow(private_interfaces)]
    pub fn new(array: Vec<UpdateTagsRegistry>) -> Self {
        Self { tag_array_length: VarInt::from(array.len() as i32), tag_array: array }
    }
}

impl ClientPacket for UpdateTags {
    fn write(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_varint(self.tag_array_length);
        
        for registry in &self.tag_array {
            buffer.put_slice(registry.encode().to_u8());
        }

        encode_packet(Self::PACKET_ID, buffer)
    }
}