#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn hello() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:9099")?;

    hc.do_get("/hello").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn create_message() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:9099")?;

    let req_create_message = hc.do_post(
        "/api/messages", 
        json!({
            "title": "sometitle",
            "content": "CONTENT YAY" 
        })
    );

    req_create_message.await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn list_messages() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:9099")?;

    let req_list = hc.do_get(
        "/api/messages"
    );

    req_list.await?.print().await?;

    Ok(())
}