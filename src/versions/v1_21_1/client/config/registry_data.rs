use crab_nbt::Nbt;

use crate::{common::Identifier, types::{packet::{ClientPacket, Packet}, ByteBuffer, VarInt}};

#[derive(Debug)]
struct RegistryDataEntry {
    entry_id: Identifier,
    has_data: bool,
    data: Option<Nbt>
}

impl RegistryDataEntry {
    pub fn encode(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_identifier(self.entry_id.clone());
        buffer.put_bool(self.has_data);
        
        if self.has_data {
            buffer.put_nbt(self.data.clone().unwrap());
        }

        buffer
    }
}

// ---
#[derive(Debug)]
pub struct RegistryData {
    registry_id: Identifier,
    entry_count: VarInt,
    entries: Vec<RegistryDataEntry>
}

impl Packet for RegistryData {
    const PACKET_ID: i32 = 0x07;
}

impl RegistryData {
    #[allow(private_interfaces)]
    pub fn new_entry(id: Identifier, data: Option<Nbt>) -> RegistryDataEntry {

        let has_data = !data.is_none();

        RegistryDataEntry { entry_id: id, has_data: has_data, data: data }
    }

    #[allow(private_interfaces)]
    pub fn new(id: Identifier, entries: Vec<RegistryDataEntry>) -> Self {

        let entry_count = entries.len();

        Self { registry_id: id, entry_count: VarInt::from(entry_count as i32), entries: entries }
    }
}

impl ClientPacket for RegistryData {
    fn get_payload(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_identifier(self.registry_id.clone());
        buffer.put_varint(self.entry_count);

        for entry in &self.entries {
            buffer.put_slice(entry.encode().to_u8());
        }

        buffer
    }
}