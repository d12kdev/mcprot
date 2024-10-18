use crate::types::VarInt;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainHand {
    Left,
    Right
}

impl From<i32> for MainHand {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Left,
            1 => Self::Right,
            _ => Self::Left
        }
    }
}

impl From<VarInt> for MainHand {
    fn from(value: VarInt) -> Self {
        Self::from(value.get_value())
    }
}