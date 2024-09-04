use std::{sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}};

use crate::{Error, Result};
use serde::{Serialize, Deserialize};
use sqlx::{postgres::PgRow, query, FromRow, PgPool, Row};

// TYPES
#[derive(Clone, Debug, Serialize, FromRow)]
pub struct Message {
    pub id: i64,
    pub time_created: i64,
    pub title: String,
    pub content: String,
}

impl Message {
    
}

#[derive(Deserialize)]
pub struct MessageCreateInfo {
    pub title: String,
    pub content: String
}

// MODEL CONTROLLER
#[derive(Clone)]
pub struct MessageController {
    db_pool: PgPool
}

impl MessageController {
    pub async fn new(db_pool: PgPool) -> Result<Self> {
        Ok(
            Self {
                db_pool
            }
        )
    }
}

impl MessageController {
    pub async fn create_message(&self, message_ci: MessageCreateInfo) -> Result<Message> {
        let start = SystemTime::now();
        let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

        let message = sqlx::query_as!(
            Message,
            "INSERT INTO messages_to_world (title, content, time_created)
            VALUES ($1, $2, $3)
            RETURNING *",
            message_ci.title,
            message_ci.content,
            since_the_epoch.as_secs() as i64
        ).fetch_one(&self.db_pool).await
        .map_err(|_| Error::DatabaseQueryError)?;

        Ok(message)
    }

    pub async fn list_messages(&self) -> Result<Vec<Message>> {

        let messages = sqlx::query_as!(
            Message,
            "select * from messages_to_world"
        ).fetch_all(&self.db_pool).await
        .map_err(|_| Error::DatabaseQueryError)?;

        Ok(messages)
    }

    pub async fn delete_message(&self, id: u64) -> Result<Message> {
        
        let deleted_message = sqlx::query_as!(
            Message,
            "DELETE FROM messages_to_world
            WHERE id = $1
            RETURNING *",
            id as i64
        ).fetch_optional(&self.db_pool).await
        .map_err(|_| Error::DatabaseQueryError)?;

        deleted_message.ok_or(Error::MessageIdNotFound{id})
    }
}
