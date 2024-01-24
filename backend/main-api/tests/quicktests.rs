#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("https://localhost:8082")?;

    hc.do_get("/hello").await?.print().await?;

    Ok(())
}