use serde::Serialize;

use crate::types::VarInt;


#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Entity {
    Allay,
    AreaEffectCloud,
    ArmorStand,
    Arrow,
    Axolotl
}

impl Entity {
    pub fn to_i32(self) -> i32 {
        self.into()
    }

    pub fn to_varint(self) -> VarInt {
        self.into()
    }
}

impl ToString for Entity {
    fn to_string(&self) -> String {
        format!("minecraft:{}", serde_json::to_string(self).unwrap().trim_matches('"'))
    }
}

impl Into<i32> for Entity {
    fn into(self) -> i32 {
        match self {
            Self::Allay => 0,
            Self::AreaEffectCloud => 1,
            Self::ArmorStand => 2,
            Self::Arrow => 3,
            Self::Axolotl => 4
        }
    }
}

impl Into<VarInt> for Entity {
    fn into(self) -> VarInt {
        let int: i32 = self.into();
        VarInt::new(int)
    }
}