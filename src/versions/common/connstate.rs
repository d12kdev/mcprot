use crate::types::VarInt;

/// Connection states
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Transfer,
    Config,
    Play,
}

impl From<VarInt> for ConnectionState {
    fn from(value: VarInt) -> Self {
        let value: i32 = value.into();
        match value {
            1 => Self::Status,
            2 => Self::Login,
            3 => Self::Transfer,
            _ => {     
                Self::Status
            }
        }
    }
}

impl Into<VarInt> for ConnectionState {
    fn into(self) -> VarInt {
        match self {
            Self::Status => VarInt::new(1),
            Self::Login => VarInt::new(2),
            Self::Transfer => VarInt::new(3),
            _ => VarInt::new(1)
        }
    }
}