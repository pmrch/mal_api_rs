pub struct SearchConfig<'a> {
    pub limit:             u32,
    pub fields:            &'a [&'a str],
    pub num_titles:        usize,
    pub threshold:         f64,
    pub first_page_offset: u32,
}

impl Default for SearchConfig<'_> {
    fn default() -> Self {
        SearchConfig {
            limit:             super::DEFAULT_LIMIT,
            fields:            super::DEFAULT_FIELDS,
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
