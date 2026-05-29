use mal_api_rs::prelude::{MalApi, Result};
use mal_api_rs::{AnimeNode, SortOrder};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let client_id: String = std::env::var("CLIENT_ID")?;
    let api: MalApi = MalApi::new(None, &client_id)?;

    // Test with lots of fields
    let fields: &[&str] = &["media_type", "status", "start_date"];
    let animes: Vec<AnimeNode> = api.anime().fields(fields).sort(SortOrder::Rank).search("Takopii no Genzai").await?;
    println!("{} animes match the title closely", animes.len());

    for anime in animes {
        println!(
            "{} | {} | {}",
            anime.title,
            anime.media_type.as_ref().unwrap(),
            anime.start_date.unwrap_or_default()
        );
    }

    Ok(())
}
