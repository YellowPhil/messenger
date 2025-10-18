use uuid::Uuid;

mod message_api;
mod room_api;

pub(crate) struct AppState {
    db: db::client::Db
}


lazy_static::lazy_static! {
    static ref SUCCESS_RESPONSE: serde_json::Value = serde_json::json!({ "success": true });
}