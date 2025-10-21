use std::sync::Arc;

use scylla::{
    client::session::Session, response::PagingState, statement::prepared::PreparedStatement,
};

use crate::models::room::{Room, RoomInfo};

mod queries {
    pub const CREATE_ROOM: &str = "INSERT INTO rooms (room_id, room_info, admin_id, created_at, updated_at) \
         VALUES (?, ?, ?, ?, ?)";

    pub const GET_ROOM_BY_ID: &str = "SELECT * FROM rooms WHERE room_id = ?";

    pub const GET_ROOMS_BY_USER: &str = "SELECT * FROM rooms_by_user WHERE user_id = ?";
}

#[derive(Debug, Clone)]
pub struct RoomRepository {
    session: Arc<Session>,
    create_statement: Arc<PreparedStatement>,
    get_statement: Arc<PreparedStatement>,
    query_user_rooms_statement: Arc<PreparedStatement>,
}

type Result<T> = std::result::Result<T, crate::errors::DbError>;

impl RoomRepository {
    pub async fn new(session: Arc<Session>) -> Result<Self> {
        let create_statement = Arc::new(session.prepare(queries::CREATE_ROOM).await?);
        let get_statement = Arc::new(session.prepare(queries::GET_ROOM_BY_ID).await?);
        let query_user_rooms_statement =
            Arc::new(session.prepare(queries::GET_ROOMS_BY_USER).await?);

        Ok(Self {
            session,
            create_statement,
            get_statement,
            query_user_rooms_statement,
        })
    }
    pub async fn create_room(
        &self,
        room_info: &RoomInfo,
        admin_id: uuid::Uuid,
    ) -> Result<uuid::Uuid> {
        let room_id = uuid::Uuid::new_v4();
        let now = chrono::Utc::now();
        self.session
            .execute_single_page(
                &self.create_statement,
                (room_id, room_info, admin_id, now, now),
                PagingState::default(),
            )
            .await
            .map_err(|e| crate::errors::DbError::ExecutionError(e))?;

        Ok(room_id)
    }
    pub async fn get_room(&self, room_id: uuid::Uuid) -> Result<Room> {
        let response = self
            .session
            .execute_single_page(&self.get_statement, (room_id,), PagingState::default())
            .await?;
        Ok(response
            .0
            .into_rows_result()
            .map_err(|e| crate::errors::DbError::QueryError(e.to_string()))?
            .single_row()
            .map_err(|e| crate::errors::DbError::QueryError(e.to_string()))?)
    }
    // pub async fn query_user_rooms(&self, user_id: uuid::Uuid) -> Result<Vec<Room>> {
    //     let response = self
    //         .session
    //         .execute_single_page(&self.query_user_rooms_statement, (user_id,), PagingState::default())
    //         .await?;

    //     let rows = response
    //         .0
    //         .into_rows_result()
    //         .map_err(|e| crate::errors::DbError::QueryError(e.to_string()))?
    //         .rows::<Room>()
    //         .map_err(|e| crate::errors::DbError::QueryError(e.to_string()))?
    //     Ok(rows)
    // }
}
