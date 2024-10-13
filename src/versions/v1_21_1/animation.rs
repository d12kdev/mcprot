use serde::Serialize;


#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Animation {
    SwingMainArm,
    LeaveBed,
    SwingOffhand,
    CriticalEffect,
    MagicCriticalEffect
}


impl Into<u8> for Animation {
    fn into(self) -> u8 {
        match self {
            Self::SwingMainArm => 0,
            Self::LeaveBed => 2,
            Self::SwingOffhand => 3,
            Self::CriticalEffect => 4,
            Self::MagicCriticalEffect => 5
        }
    }
}