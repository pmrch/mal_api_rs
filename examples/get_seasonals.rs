use mal_api_rs::models::SeasonEnum;
use mal_api_rs::{Anime, MalApi, Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let client_id: String = std::env::var("CLIENT_ID")?;
    let api: MalApi = MalApi::new(None, &client_id)?;

    let fields: &[&str] = &["start_date", "status", "media_type", "alternative_titles"];
    let seasonals: Vec<Anime> = api.anime().limit(500).fields(fields.iter()).seasonal(2026, SeasonEnum::Summer).await?;

    println!("Got {} seasonals", seasonals.len());
    for sea in seasonals {
        println!(
            "{} | {} | {} | {} | {}",
            sea.node.title,
            sea.node.start_date.unwrap(),
            sea.node.status.unwrap(),
            sea.node.media_type.unwrap(),
            sea.node.alternative_titles.map(|at| at.en).unwrap()
        );
    }

    Ok(())
}
