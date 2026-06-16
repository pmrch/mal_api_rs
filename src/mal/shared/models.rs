#[cfg(feature = "user")]
pub use crate::mal::user::{Statistics, UserInfo};

pub struct SearchConfig {
    pub fields:            Vec<compact_str::CompactString>,
    pub limit:             u32,
    pub num_titles:        usize,
    pub threshold:         f64,
    pub first_page_offset: u32,
}

impl Default for SearchConfig {
    fn default() -> Self {
        let default_fields: Vec<_> = Vec::from(&["start_date".into(), "alternative_title".into()]);
        Self {
            fields:            default_fields,
            limit:             super::DEFAULT_LIMIT,
            num_titles:        super::DEFAULT_NUM_TITLES,
            threshold:         super::DEFAULT_THRESHOLD,
            first_page_offset: super::DEFAULT_OFFSET,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SearchMode {
    All,
    Any,
    AtLeast(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum SortOrder {
    Title,
    StartDate,
    MeanScore,
    Popularity,
    Rank,
}

#[derive(Debug, Clone, Default)]
pub enum QuerySort {
    ListScore,
    ListUpdatedAt,
    #[default]
    AnimeTitle,
    AnimeStartDate,
    AnimeId,
}

/// Between(min, max) - inclusive range. If min > max, they'll be swapped.
#[derive(Debug, Clone, Copy)]
pub enum NumEps {
    LessThan(u32),
    MoreThan(u32),
    Exactly(u32),
    Between(u32, u32),
}

#[derive(Debug, Clone, Copy)]
pub enum EpLengthMins {
    ShorterThan(u32),
    LongerThan(u32),
    Between(u32, u32),
}
