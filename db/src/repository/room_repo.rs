use std::sync::Arc;

use scylla::{
    client::session::Session, response::PagingState, statement::prepared::PreparedStatement,
};

use crate::models::room::Room;

#[derive(Debug, Clone)]
pub struct RoomRepository {
    session: Arc<Session>,
    create_statement: Arc<PreparedStatement>,
    list_statement: Arc<PreparedStatement>,
    get_statement: Arc<PreparedStatement>,
    update_statement: Arc<PreparedStatement>,
    delete_statement: Arc<PreparedStatement>,
    get_by_name_statement: Arc<PreparedStatement>,
    get_user_rooms_statement: Arc<PreparedStatement>,
}

type Result<T> = std::result::Result<T, crate::errors::DbError>;

impl RoomRepository {
    pub async fn new(session: Arc<Session>) -> Result<Self> {
        let create_statement = Arc::new(session.prepare("INSERT INTO rooms (id, name, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?)").await?);
        let list_statement = Arc::new(session.prepare("SELECT * FROM rooms").await?);
        let get_statement = Arc::new(session.prepare("SELECT * FROM rooms WHERE id = ?").await?);
        let get_by_name_statement = Arc::new(
            session
                .prepare("SELECT * FROM rooms WHERE name = ?")
                .await?,
        );
        let get_user_rooms_statement = Arc::new(
            session
                .prepare("SELECT * FROM rooms WHERE admin_id = ?")
                .await?,
        );
        let update_statement = Arc::new(
            session
                .prepare("UPDATE rooms SET name = ?, description = ?, updated_at = ? WHERE id = ?")
                .await?,
        );
        let delete_statement = Arc::new(session.prepare("DELETE FROM rooms WHERE id = ?").await?);
        Ok(Self {
            session,
            create_statement,
            list_statement,
            get_statement,
            update_statement,
            delete_statement,
            get_by_name_statement,
            get_user_rooms_statement,
        })
    }
    pub async fn create_room(&self, room: Room) -> Result<uuid::Uuid> {
        let room_id = uuid::Uuid::new_v4();

        self.session
            .execute_single_page(
                &self.create_statement,
                (
                    room_id,
                    room.name,
                    room.description,
                    room.created_at,
                    room.updated_at,
                ),
                PagingState::default(),
            )
            .await?;

        Ok(room_id)
    }
    pub async fn get_room(&self, room_id: uuid::Uuid) -> Result<Room> {
        let response = self
            .session
            .execute_single_page(&self.get_statement, (room_id,), PagingState::default())
            .await?;
        Ok(response.0.into_rows_result()?.single_row())
    }
}
