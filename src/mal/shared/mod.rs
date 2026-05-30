pub mod filter;
pub mod impls;
pub mod models;
pub mod api;
pub mod traits;

const DEFAULT_FIELDS: &[&str] = &["start_date", "alternative_titles"];
const DEFAULT_THRESHOLD: f64 = 0.75f64;
const DEFAULT_NUM_TITLES: usize = 15;
const DEFAULT_LIMIT: u32 = 50;
const DEFAULT_OFFSET: u32 = 0;
