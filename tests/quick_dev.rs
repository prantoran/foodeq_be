
use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/hello?name=pinku").await?.print().await?;
    hc.do_get("/hello2/prantoran").await?.print().await?;
    hc.do_get("/pub/welcome.txt").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo",
            "password": "123",
        }),
    );
    req_login.await?.print().await?;

    // Checking cookie set by login
    hc.do_get("/hello?name=pinku").await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Test Ticket",
        }),
    );
    req_create_ticket.await?.print().await?;

    hc.do_get("/api/tickets")
        .await?
        .print()
        .await?;

    hc.do_delete("/api/tickets/3")
        .await?
        .print()
        .await?;
    Ok(())
}