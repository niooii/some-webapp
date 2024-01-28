use crate::models::message::{MessageModelController, Message, MessageCreateInfo};
use crate::Result;
use axum::extract::{FromRef, Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

pub fn routes(mc: MessageModelController) -> Router {
    Router::new()
    .route("/messages", post(create_message).get(list_messages))
    .route("/messages/:id", delete(delete_message))
    .with_state(mc)
} 

async fn create_message(
    State(mc): State<MessageModelController>, 
    Json(message_ci): Json<MessageCreateInfo>
) -> Result<Json<Message>> {

    println!("->> {:<12} - create_message", "HANDLER");

    let message = mc.create_message(message_ci).await?;

    Ok(Json(message))
}

async fn list_messages(
    State(mc): State<MessageModelController>,
) -> Result<Json<Vec<Message>>> {

    println!("->> {:<12} - list_messages", "HANDLER");

    let messages = mc.list_messages().await?;

    Ok(Json(messages))
}

async fn delete_message(
    State(mc): State<MessageModelController>, Path(id): Path<u64>,
) -> Result<Json<Message>> {
    println!("->> {:<12} - delete_message", "HANDLER");

    let message = mc.delete_message(id).await?;

    Ok(Json(message))
}

