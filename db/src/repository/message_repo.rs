use scylla::{client::session::Session, statement::prepared::PreparedStatement};
use std::sync::Arc;

use crate::{models::message::Message};

#[derive(Debug, Clone)]
pub struct MessageRepository {
    session: Arc<Session>,
    create_statement: Arc<PreparedStatement>,
    list_statement: Arc<PreparedStatement>
}

type Result<T> = std::result::Result<T, crate::errors::DbError>;

impl MessageRepository {
    pub async fn new(session: Arc<Session>) -> Result<Self> {
        let create_statement = 
        Arc::new(session.prepare("INSERT INTO messages (id, from_user_id, to_user_id, content) VALUES (?, ?, ?, ?)").await?);
        let list_statement = Arc::new(session.prepare("SELECT * FROM messages WHERE id = ?").await?);
        Ok(Self { session, create_statement, list_statement })
    }
    pub async fn create_message(&self, message: Message) -> Result<Message> {
        todo!()
    }
}
