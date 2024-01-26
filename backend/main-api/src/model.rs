use crate::{Error, Result};
use serde::{Serialize, Deserialize};


pub struct Message {
    pub id: u64,
    pub time_created: u64,
    pub title: String,
    pub content: String,
}

pub struct MessageCreateInfo {
    pub title: String,
    pub content: String
}