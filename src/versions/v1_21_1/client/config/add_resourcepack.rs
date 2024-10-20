use uuid::Uuid;

use crate::{common::text::TextComponent, types::{packet::{ClientPacket, Packet}, ByteBuffer}};


#[derive(Debug)]
pub struct AddResourcepack {
    uuid: Uuid,
    url: String,
    hash: String,
    forced: bool,
    has_prompt_message: bool,
    prompt_message: Option<TextComponent>
}

impl Packet for AddResourcepack {
    const PACKET_ID: i32 = 0x09;
}

impl AddResourcepack {
    pub fn new(uuid: Uuid, url: String, hash: String, forced: bool, prompt_message: Option<TextComponent>) -> Self {
        let has_prompt_message = !prompt_message.is_none();

        Self {
            uuid,
            url,
            hash,
            forced,
            has_prompt_message,
            prompt_message
        }
    }
}

impl ClientPacket for AddResourcepack {
    fn get_payload(&self) -> crate::types::ByteBuffer {
        let mut buffer = ByteBuffer::new();
        buffer.put_uuid(self.uuid);
        buffer.put_string(self.url.clone()).unwrap();
        buffer.put_string(self.hash.clone()).unwrap();
        buffer.put_bool(self.forced);
        buffer.put_bool(self.has_prompt_message);

        if self.has_prompt_message {
            buffer.put_textcomponent(self.prompt_message.clone().unwrap());
        }

        buffer
    }
}