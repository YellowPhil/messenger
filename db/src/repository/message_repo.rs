use scylla::{client::session::Session, statement::prepared::PreparedStatement};
use std::sync::Arc;

use crate::{models::message::Message};

mod queries {
    pub const CREATE_MESSAGE: &str = 
        "INSERT INTO messages (id, from_user_id, to_user_id, content) \
         VALUES (?, ?, ?, ?)";
    
    pub const GET_MESSAGE_BY_ID: &str = 
        "SELECT * FROM messages WHERE id = ?";
}

#[derive(Debug, Clone)]
pub struct MessageRepository {
    session: Arc<Session>,
    create_statement: Arc<PreparedStatement>,
    list_statement: Arc<PreparedStatement>
}

type Result<T> = std::result::Result<T, crate::errors::DbError>;

impl MessageRepository {
    pub async fn new(session: Arc<Session>) -> Result<Self> {
        let create_statement = Arc::new(
            session.prepare(queries::CREATE_MESSAGE).await?
        );
        let list_statement = Arc::new(
            session.prepare(queries::GET_MESSAGE_BY_ID).await?
        );
        
        Ok(Self { 
            session, 
            create_statement, 
            list_statement 
        })
    }
    pub async fn create_message(&self, message: Message) -> Result<Message> {
        todo!()
    }
}
