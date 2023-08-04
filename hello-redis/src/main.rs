use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("114.96.104.144:19001").await?;
    client.set("hello", "feitan".into()).await?;
    let reset = client.get("hello").await?;
    println!("got value from the server; result={:?}", reset);
    Ok(())
}