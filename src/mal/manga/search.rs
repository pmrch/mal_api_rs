use super::endpoints::MANGA_ENDPOINT;
use super::helpers::{build_fields, check_response, matches_title, sort_vec};
use super::models::{MangaNode, MangaQuery};
use super::requests::{Client, Response};
use super::shared_models::{AnimeNode, SearchConfig, SearchFilter, SearchMode, SortOrder};
use super::{Arc, CompactString, HashMap, HashSet, Result, Url};

pub struct MangaSearchBuilder<'a> {
    client:        Arc<Client>,
    access_token:  Option<Arc<str>>,
    config:        SearchConfig<'a>,
    search_mode:   SearchMode,
    search_filter: Option<SearchFilter>,
    sorting_order: Option<SortOrder>,
}

impl<'a> MangaSearchBuilder<'a> {
    pub fn new(client: Arc<Client>, access_token: Option<Arc<str>>) -> Self {
        Self {
            client,
            access_token,
            config: SearchConfig::default(),
            search_mode: SearchMode::All,
            search_filter: None,
            sorting_order: None,
        }
    }

    /// Set the similarity threshold for title comparison with query
    pub const fn threshold(mut self, threshold: f64) -> Self {
        self.config.threshold = threshold;
        self
    }

    /// Set the fields that should get queried and deserialized
    pub const fn fields(mut self, fields: &'a [&'a str]) -> Self {
        self.config.fields = fields;
        self
    }

    /// Set the number of matches you wish to receive
    pub const fn number_of_titles(mut self, num_titles: usize) -> Self {
        self.config.num_titles = num_titles;
        self
    }

    /// Set the limit of anime per page
    pub const fn limit(mut self, limit: u32) -> Self {
        self.config.limit = limit;
        self
    }

    /// If `true`, results are sorted alphabetically by title.
    /// Defaults to `false`.
    pub const fn sort(mut self, order: SortOrder) -> Self {
        self.sorting_order = Some(order);
        self
    }

    /// Set the offset for the page of queries
    pub const fn first_page_offset(mut self, offset: u32) -> Self {
        self.config.first_page_offset = offset;
        self
    }

    /// Controls how multiple filters are evaluated against each result.
    /// - [`SearchMode::All`] — all filters must match (AND logic)
    /// - [`SearchMode::Any`] — at least one filter must match (OR logic)
    /// - [`SearchMode::AtLeast(n)`] — at least `n` filters must match
    ///
    /// Optional, defaults to [`SearchMode::All`]
    pub const fn filter_match_mode(mut self, search_mode: SearchMode) -> Self {
        self.search_mode = search_mode;
        self
    }

    /// Set filters to refine search results beyond title matching.
    /// Use [`AnimeSearchBuilder::filter_match_mode`] to control how
    /// multiple filters are evaluated. Optional, no filtering by default.
    pub fn filter(mut self, filter: SearchFilter) -> Self {
        self.search_filter = Some(filter);
        self
    }

    pub async fn search(&self, title: impl AsRef<str>) -> Result<Vec<MangaNode>> {
        let new_fields: Vec<&'a str> = build_fields(self.config.fields, &self.sorting_order, &self.search_filter);
        let query_params: HashMap<&'a str, CompactString> = crate::my_hash_map! {
            "q" => title.as_ref(),
            "offset" => self.config.first_page_offset,
            "limit" => self.config.limit,
            "nsfw" => "true",
            "fields" => &new_fields.join(",")
        };

        let url: Url = Url::parse_with_params(&MANGA_ENDPOINT, query_params)?;
        let response: Response = self.client.get(url).send().await?;
        check_response(response.status())?;

        let mut target_query: HashSet<MangaNode> = HashSet::with_capacity(self.config.num_titles);
        let mut pages_without_match: usize = 0;

        let resp_val: serde_json::Value = response.json().await?;
        let mut response: MangaQuery = serde_json::from_value(resp_val)?;

        loop {
            for entry in response.data {
                let filter_matches: bool = self.search_filter.as_ref().is_none_or(|f| f.matches(&entry, &self.search_mode));
                if matches_title(&entry, title.as_ref(), self.config.threshold) && filter_matches {
                    pages_without_match = 0;
                    if target_query.len() >= self.config.num_titles {
                        break;
                    }

                    target_query.insert(entry);
                }
            }

            if target_query.len() >= self.config.num_titles || pages_without_match >= 8 {
                break;
            }

            let next_page = match &response.paging.next {
                Some(next) => {
                    if next.is_empty() {
                        break;
                    }

                    next
                }
                None => break,
            };

            let Ok(url_maybe_last) = url::Url::parse(next_page) else {
                eprintln!("MAL returned malformed next page URL: {next_page}");
                break;
            };

            let resp: Response = self.client.get(url_maybe_last).send().await?;
            if check_response(resp.status()).is_err() {
                tracing::error!("An error response was received for a page");
                break;
            }

            response = resp.json::<MangaQuery>().await?;
            pages_without_match += 1;
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }

        let mut mangas: Vec<MangaNode> = target_query.into_iter().collect();
        self.sort_vec(&mut mangas);

        Ok(mangas)
    }

    /// Sort a slice of anime by the configured sort order.
    fn sort_vec(&self, input: &mut [MangaNode]) {
        if let Some(sort_order) = &self.sorting_order {
            input.sort_by(|a, b| match sort_order {
                SortOrder::Title => a.title.cmp(&b.title),
                SortOrder::MeanScore => a.mean.cmp(&b.mean),
                SortOrder::StartDate => a.start_date.cmp(&b.start_date),
                SortOrder::Popularity => a.popularity.cmp(&b.popularity),
                SortOrder::Rank => a.rank.cmp(&b.rank),
            });
        }
    }
}
