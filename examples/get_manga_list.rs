use mal_api_rs::models::{MangaNode, MangaRankingType};
use mal_api_rs::{MalApi, Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let client_id: String = std::env::var("CLIENT_ID")?;
    let api: MalApi = MalApi::new(None, &client_id)?;

    let fields: &[&str] = &["media_type", "status", "start_date", "authors"];
    let mangas: Vec<MangaNode> = api.manga().fields(fields.iter()).limit(1).search("Kimetsu no Yaiba").await?;
    for manga in mangas {
        println!("{manga}");
    }

    let ranking: Vec<mal_api_rs::models::MangaRankingQueryData> =
        api.manga().fields(["nsfw", "start_date", "rank"].iter()).limit(20).get_ranking(MangaRankingType::All).await?;

    for ranked in ranking {
        println!("{ranked}");
    }

    Ok(())
}
