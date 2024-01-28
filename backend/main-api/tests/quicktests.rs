#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn test_login() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:9099")?;

    // hc.do_post(
    //     "/api/login",
    //     json!({
    //         "username": "demo1",
    //         "pwd": "welcome"
    //     })
    // ).await?.print().await?;

    let req_create_message = hc.do_post(
        "/api/messages", 
        json!({
            "title": "sometitle",
            "content": "CONTENT YAY" 
        })
    );

    req_create_message.await?.print().await?;

    let req_list_messages = hc.do_get(
        "/api/messages"
    );

    req_list_messages.await?.print().await?;

    let req_delete_message = hc.do_delete(
        "/api/messages/3"
    );

    req_delete_message.await?.print().await?;

    Ok(())
}

// #[tokio::test]
// async fn create_message() -> Result<()> {
//     let hc = httpc_test::new_client("http://localhost:9099")?;

//     let req_create_message = hc.do_post(
//         "/api/messages", 
//         json!({
//             "title": "sometitle",
//             "content": "CONTENT YAY" 
//         })
//     );

//     req_create_message.await?.print().await?;

//     Ok(())
// }

// #[tokio::test]
// async fn list_messages() -> Result<()> {
//     let hc = httpc_test::new_client("http://localhost:9099")?;

//     let req_list = hc.do_get(
//         "/api/messages"
//     );

//     req_list.await?.print().await?;

//     Ok(())
// }