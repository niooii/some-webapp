use std::{sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}};

use crate::{Error, Result};
use serde::{Serialize, Deserialize};

// TYPES
#[derive(Clone, Debug, Serialize)]
pub struct Message {
    pub id: u64,
    pub time_created: u64,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct MessageCreateInfo {
    pub title: String,
    pub content: String
}

// MODEL CONTROLLER
#[derive(Clone)]
pub struct ModelController {
    message_store: Arc<Mutex<Vec<Option<Message>>>>
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(
            Self {
                message_store: Arc::default()
            }
        )
    }
}

impl ModelController {
    pub async fn create_message(&self, message_ci: MessageCreateInfo) -> Result<Message> {
        let mut store = self.message_store.lock().unwrap();

        let start = SystemTime::now();
        let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

        let id = store.len() as u64;
        let message = Message {
            id,
            time_created: since_the_epoch.as_secs(),
            title: message_ci.title,
            content: message_ci.content 
        };
    
        store.push(Some(message.clone()));

        Ok(message)
    }

    pub async fn list_messages(&self) -> Result<Vec<Message>> {
        let store = self.message_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_message(&self, id: u64) -> Result<Message> {
        let mut store = self.message_store.lock().unwrap();

        let message = store.get_mut(id as usize).and_then(|m| m.take());

        message.ok_or(Error::MessageIdNotFound{id})
    }
}
