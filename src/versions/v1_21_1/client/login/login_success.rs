use uuid::Uuid;

use crate::types::{packet::Packet, VarInt};

#[derive(Debug)]
pub struct LoginSuccessProperty {
    pub name: String, // String(32767)
    pub value: String, // String(32767)
    pub is_signed: bool,
    pub signature: Option<String> // String(32767)
}

impl LoginSuccessProperty {
    pub fn new(name: String, value: String, is_signed: bool, signature: Option<String>) -> Self {
        Self {
            name,
            value,
            is_signed,
            signature
        }
    }
}

#[derive(Debug)]
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