mod error;
mod macros;
mod mal;
pub mod prelude;

pub use mal::{QuerySort, SearchConfig, models};
pub use prelude::{Anime, MalApi, Result, SearchFilter, SortOrder};
