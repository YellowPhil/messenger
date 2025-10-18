use scylla::DeserializeRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, DeserializeRow)]
pub struct Room {
    room_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub admin_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[scylla(skip)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}