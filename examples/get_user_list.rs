use mal_api_rs::models::UserAnimeListEdge;
use mal_api_rs::{MalApi, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    dotenv::dotenv()?;
    let client_id: String = std::env::var("CLIENT_ID")?;
    let api: MalApi = MalApi::new(None, &client_id)?;

    let results: Vec<UserAnimeListEdge> = api.user_anime().limit(1000).get(Some("Patrikgamer2000")).await?;
    println!("Found {} entries in Patrikgamer2000's list", results.len());
    println!("{}\nlist status: {:?}", results[0].node, results[0].list_status);
    Ok(())
}
