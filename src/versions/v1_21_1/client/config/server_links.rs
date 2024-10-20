use color_eyre::eyre::Result;

use crate::{common::text::TextComponent, types::{packet::{ClientPacket, Packet}, ByteBuffer, VarInt}};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SLLType {
    ServerLinkLabel(ServerLinkLabel),
    TextComponent(TextComponent)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ServerLinkLabel {
    BugReport,
    CommunityGuidelines,
    Support,
    Status,
    Feedback,
    Community,
    Website,
    Forums,
    News,
    Annoucements
}

impl From<i32> for ServerLinkLabel {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::BugReport,
            1 => Self::CommunityGuidelines,
            2 => Self::Support,
            3 => Self::Status,
            4 => Self::Feedback,
            5 => Self::Community,
            6 => Self::Website,
            7 => Self::Forums,
            8 => Self::News,
            9 => Self::Annoucements,
            _ => Self::BugReport
        }
    }
}

impl Into<i32> for ServerLinkLabel {
    fn into(self) -> i32 {
        match self {
            ServerLinkLabel::BugReport => 0,
            ServerLinkLabel::CommunityGuidelines => 1,
            ServerLinkLabel::Support => 2,
            ServerLinkLabel::Status => 3,
            ServerLinkLabel::Feedback => 4,
            ServerLinkLabel::Community => 5,
            ServerLinkLabel::Website => 6,
            ServerLinkLabel::Forums => 7,
            ServerLinkLabel::News => 8,
            ServerLinkLabel::Annoucements => 9,
        }
    }
}


#[derive(Debug)]
pub struct ServerLink {
    pub is_builtin: bool,
    pub label: SLLType,
    pub url: String
}

impl ServerLink {
    pub fn new_textlabel(is_builtin: bool, label: TextComponent, url: String) -> Self {
        Self {
            is_builtin,
            label: SLLType::TextComponent(label),
            url
        }
    }

    pub fn new_enumlabel(is_builtin: bool, label: ServerLinkLabel, url: String) -> Self {
        Self {
            is_builtin,
            label: SLLType::ServerLinkLabel(label),
            url
        }
    }

    pub fn encode(&self) -> Result<ByteBuffer> {
        let mut buffer = ByteBuffer::new();

        buffer.put_bool(self.is_builtin);
        
        match self.label.clone() {
            SLLType::ServerLinkLabel(lab) => {
                buffer.put_varint(VarInt::new(lab.into()));
            },
            SLLType::TextComponent(component) => {
                buffer.put_textcomponent(component);
            }
        }

        buffer.put_string(self.url.clone())?;

        Ok(buffer)
    }
}


#[derive(Debug)]
pub struct ServerLinks {
    links_count: VarInt,
    links: Vec<ServerLink>
}

impl Packet for ServerLinks {
    const PACKET_ID: i32 = 0x10;
}

impl ServerLinks {
    pub fn new(links: Vec<ServerLink>) -> Self {
        Self {
            links_count: VarInt::from(links.len()),
            links
        }
    }
}

impl ClientPacket for ServerLinks {
    fn get_payload(&self) -> ByteBuffer {
        let mut buffer = ByteBuffer::new();

        buffer.put_varint(self.links_count);
        
        for link in &self.links {
            buffer.put_slice(link.encode().unwrap().to_u8());
        }

        buffer
    }
}