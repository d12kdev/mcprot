use uuid::Uuid;

use crate::types::{packet::{ClientPacket, Packet}, ByteBuffer, VarInt};

#[derive(Debug, Clone)]
struct LoginSuccessProperty {
    pub name: String, // String(32767)
    pub value: String, // String(32767)
    pub is_signed: bool,
    pub signature: Option<String> // String(32767)
}

impl LoginSuccessProperty {
    pub fn encode(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_string(self.name.clone()).unwrap();
        buffer.put_string(self.value.clone()).unwrap();
        buffer.put_bool(self.is_signed);

        if self.is_signed {
            buffer.put_string(self.signature.clone().unwrap()).unwrap();
        }

        buffer
    }
}

#[derive(Debug)]
#[allow(private_interfaces)]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String, // String(16)
    pub number_of_properties: VarInt, // Number of elements in property array
    pub property: Vec<LoginSuccessProperty>,
    pub strict_error_handling: bool, // 1.20.5; will be removed in 1.21.2
}

impl Packet for LoginSuccess {
    const PACKET_ID: i32 = 0x02;
}

impl LoginSuccess {
    #[allow(private_interfaces)]
    pub fn new_property(name: String, value: String, signature: Option<String>) -> LoginSuccessProperty {
        
        let is_signed = !signature.is_none();
        
        LoginSuccessProperty {
            name,
            value,
            is_signed,
            signature
        }
    }

    #[allow(private_interfaces)]
    pub fn new(uuid: Uuid, username: String, properties: Vec<LoginSuccessProperty>, strict_error_handling: bool) -> Self {
        Self {
            uuid,
            username,
            number_of_properties: VarInt::from(properties.len() as i32),
            property: properties,
            strict_error_handling
        }
    }
}

impl ClientPacket for LoginSuccess {
    fn get_payload(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_uuid(self.uuid);
        buffer.put_string(self.username.clone()).unwrap();
        buffer.put_varint(self.number_of_properties);

        for property in &self.property {
            buffer.put_slice(property.encode().to_u8());
        }

        buffer.put_bool(self.strict_error_handling);

        buffer
    }
}