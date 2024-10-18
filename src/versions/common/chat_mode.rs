use crate::types::VarInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatMode {
    Full,
    CommandsOnly,
    Hidden
}

impl Into<i32> for ChatMode {
    fn into(self) -> i32 {
        match self {
            Self::Full => 0,
            Self::CommandsOnly => 1,
            Self::Hidden => 2
        }
    }
}

impl Into<VarInt> for ChatMode {
    fn into(self) -> VarInt {
        VarInt::new(self.into())
    }
}

impl From<i32> for ChatMode {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Full,
            1 => Self::CommandsOnly,
            2 => Self::Hidden,
            _ => Self::Full
        }
    }
}

impl From<VarInt> for ChatMode {
    fn from(value: VarInt) -> Self {
        Self::from(value.get_value())
    }
}