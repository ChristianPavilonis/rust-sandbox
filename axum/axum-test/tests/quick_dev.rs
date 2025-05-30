use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello2/tissy").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "tissy",
            "password": "secret"
        }),
    );

    req_login.await?.print().await?;

    Ok(())
}
