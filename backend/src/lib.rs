use uuid::Uuid;

mod message_api;
mod room_api;

pub(crate) struct AppState {
    db: db::client::Db
}

pub(crate) const USER_ID_KEY: &str = "user_id";