use serde::Serialize;
use server_status::{ServerDescription, ServerPlayers, ServerVersion};

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct ServerStatus {
    pub version: ServerVersion,
    pub players: ServerPlayers,
    pub description: ServerDescription,
    pub favicon: String,
    pub enforcesSecureChat: bool
}

impl ServerStatus {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub mod server_status {
    use serde::Serialize;
    use uuid::Uuid;

    #[derive(Debug, Serialize)]
    pub struct ServerVersion {
        pub name: String,
        pub protocol: i32
    }

    #[derive(Debug, Serialize)]
    pub struct ServerPlayers {
        pub max: i32,
        pub online: i32,
        pub sample: Vec<ServerPlayer>
    }

    #[derive(Debug, Serialize)]
    pub struct ServerPlayer {
        pub name: String,
        pub id: Uuid
    }

    #[derive(Debug, Serialize)]
    pub struct ServerDescription {
        pub text: String
    }
}