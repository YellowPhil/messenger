use std::sync::Arc;

use scylla::client::{session::Session, session_builder::SessionBuilder};

use crate::repository::{message_repo::MessageRepository, room_repo::RoomRepository};

#[derive(Debug, Clone)]
pub struct Db {
    connection: Arc<Session>,
    message_repo: MessageRepository,
    room_repo: RoomRepository,
}

const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

impl Db {
    async fn new(connection: Session) -> Result<Self, crate::errors::DbError>{
        let connection = Arc::new(connection);
        let message_repo = MessageRepository::new(connection.clone()).await?;
        let room_repo = RoomRepository::new(connection.clone()).await?;
        Ok(Self {
            connection,
            message_repo,
            room_repo,
        })
    }
    pub async fn from_uri(
        uri: &str,
        keyspace: &str,
        duration: Option<std::time::Duration>,
    ) -> Result<Self, crate::errors::DbError> {
        let connection = SessionBuilder::new()
            .known_node(uri)
            .use_keyspace(keyspace, true)
            .connection_timeout(duration.unwrap_or(DEFAULT_TIMEOUT))
            .build()
            .await?;

        Self::new(connection).await
    }
    pub(crate) fn get_session(&self) -> Arc<Session> {
        self.connection.clone()
    }
    pub fn messages(&self) -> &MessageRepository {
        &self.message_repo
    }
    pub fn rooms(&self) -> &RoomRepository {
        &self.room_repo
    }
}
