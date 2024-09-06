use crate::model::message::{Message, MessageController, MessageCreateInfo, MessageFetchInfo};
use crate::Result;
use axum::extract::{FromRef, Path, Query, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

pub fn routes(mc: MessageController) -> Router {
    Router::new()
    .route("/messages", post(create_message).get(list_messages))
    .route("/messages/:id", delete(delete_message))
    .with_state(mc)
} 

async fn create_message(
    State(mc): State<MessageController>, 
    Json(create_info): Json<MessageCreateInfo>
) -> Result<Json<Message>> {
    let message = mc.create_message(create_info).await?;

    Ok(Json(message))
}

async fn list_messages(
    State(mc): State<MessageController>,
    Query(fetch_info): Query<MessageFetchInfo>
) -> Result<Json<Vec<Message>>> {
    let messages = mc.list_messages(fetch_info).await?;

    Ok(Json(messages))
}

async fn delete_message(
    State(mc): State<MessageController>, Path(id): Path<u64>,
) -> Result<Json<Message>> {
    let message = mc.delete_message(id).await?;

    Ok(Json(message))
}

