use crate::{common::{ChatMode, MainHand}, types::packet::{Packet, ServerPacket}};


#[derive(Debug)]
pub struct ClientInfo {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: MainHand,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool
}

impl Packet for ClientInfo {
    const PACKET_ID: i32 = 0x00;
}

impl ServerPacket for ClientInfo {
    fn decode(bytebuffer: &mut crate::types::ByteBuffer) -> color_eyre::eyre::Result<Self> {
        Ok(
            Self {
                locale: bytebuffer.get_string_maxsize(16)?,
                view_distance: bytebuffer.get_i8(),
                chat_mode: ChatMode::from(bytebuffer.get_varint()?),
                chat_colors: bytebuffer.get_bool(),
                displayed_skin_parts: bytebuffer.get_u8(),
                main_hand: MainHand::from(bytebuffer.get_varint()?),
                enable_text_filtering: bytebuffer.get_bool(),
                allow_server_listings: bytebuffer.get_bool()
            }
        )
    }
}