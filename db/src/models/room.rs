use scylla::{DeserializeRow, DeserializeValue, SerializeRow, SerializeValue};
use serde::{Deserialize, Serialize};

use crate::models::roles::Role;

#[derive(Debug, Serialize, Deserialize, DeserializeRow, SerializeRow)]
pub struct Room {
    room_id: uuid::Uuid,
    room_info: RoomInfo,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[scylla(skip)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, DeserializeValue, SerializeValue)]
pub struct RoomInfo {
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    #[serde(rename = "role")]
    pub role: i32,
}