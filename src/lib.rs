mod error;
mod macros;
mod mal;
pub mod prelude;

pub use prelude::models::{AlternativeTitles, Anime, AnimeNode, AnimeQuery, AnimeRankingType};
pub use prelude::{MalApi, Result, SearchFilter, SortOrder};
