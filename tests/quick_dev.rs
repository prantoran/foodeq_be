
use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/hello?name=pinku").await?.print().await?;
    hc.do_get("/hello2/prantoran").await?.print().await?;
    hc.do_get("/pub/welcome.txt").await?.print().await?;

    Ok(())
}