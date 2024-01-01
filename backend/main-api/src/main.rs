use db_interface::database::{
    self, 
    message_to_world::MessageToWorld
};

use std::time::{
    SystemTime,
    UNIX_EPOCH
};

use std::sync::Arc;
use std::{
    error::Error,
    fs::{self, File},
};
use anyhow::Result;
use dotenv::dotenv;

// use simplelog::{CombinedLogger, LevelFilter, SimpleLogger, WriteLogger};

#[tokio::main]
async fn main() -> Result<()> {

    const DB_URL: &str = "postgres://niooi:abcde@localhost:9432/postgres";

    db_interface::init_pool().await?;

    let testmsg = MessageToWorld {
        title: "SOME TITLE".to_string(),
        content: "danile park gay".to_string(),
        time_created: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64() as i64
    };

    database::message_to_world::save(&testmsg).await?;

    // let pool = sqlx::postgres::PgPool::connect(DB_URL).await?;

    // 

    Ok(())
}