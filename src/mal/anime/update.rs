use super::models::ListStatusEnum;

/// Builder for updating an anime entry on the authenticated user's MAL list.
/// Only fields that are explicitly set will be included in the update request.
/// Unset fields will remain unchanged on MAL's side.
#[derive(Default)]
pub struct UpdateBuilder {
    status:              Option<ListStatusEnum>,
    score:               Option<u8>, // MAL uses 0-10
    num_watched:         Option<u32>,
    is_rewatching:       Option<bool>,
    priority:            Option<u8>, // 0-2
    num_times_rewatched: Option<u32>,
    rewatch_value:       Option<u8>, // 0-5
    tags:                Option<String>,
    comments:            Option<String>,
}

impl UpdateBuilder {
    pub fn new() -> Self { Self::default() }

    /// Set the watching status of the anime on the user's list.
    pub const fn status(mut self, status: ListStatusEnum) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the number of episodes watched.
    pub const fn watched(mut self, watched_eps: u32) -> Self {
        self.num_watched = Some(watched_eps);
        self
    }

    /// Set whether the user is currently rewatching the anime.
    pub const fn rewatching(mut self, is_rewatching: bool) -> Self {
        self.is_rewatching = Some(is_rewatching);
        self
    }

    /// Set the number of times the user has rewatched the anime.
    pub const fn num_times_rewatched(mut self, times_rewatched: u32) -> Self {
        self.num_times_rewatched = Some(times_rewatched);
        self
    }

    /// Set the rewatch value of the anime. Clamped to `0-5`.
    pub fn rewatch_value(mut self, rewatch_value: u8) -> Self {
        self.rewatch_value = Some(rewatch_value.min(5));
        self
    }

    /// Set the user's tags for this anime entry.
    pub fn tags(mut self, tags: String) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Set the user's comments for this anime entry.
    pub fn comments(mut self, comments: String) -> Self {
        self.comments = Some(comments);
        self
    }

    /// Set the priority of this anime entry. Clamped to `0-2`.
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority.min(2));
        self
    }

    /// Set the user's score for this anime. Clamped to `0-10`.
    pub fn score(mut self, score: u8) -> Self {
        self.score = Some(score.min(10));
        self
    }

    pub(crate) fn into_params(self) -> Vec<(&'static str, compact_str::CompactString)> {
        let mut params: Vec<(&str, compact_str::CompactString)> = vec![];

        if let Some(status) = self.status {
            params.push(("status", compact_str::format_compact!("{status}")));
        }

        if let Some(score) = self.score {
            params.push(("score", compact_str::format_compact!("{score}")));
        }

        if let Some(num_watched) = self.num_watched {
            params.push(("num_watched", compact_str::format_compact!("{num_watched}")));
        }

        if let Some(is_rewatching) = self.is_rewatching {
            params.push(("is_rewatching", compact_str::format_compact!("{is_rewatching}")));
        }

        if let Some(priority) = self.priority {
            params.push(("priority", compact_str::format_compact!("{priority}")));
        }

        if let Some(num_times_rewatched) = self.num_times_rewatched {
            params.push(("num_times_rewatched", compact_str::format_compact!("{num_times_rewatched}")));
        }

        if let Some(rewatch_value) = self.rewatch_value {
            params.push(("rewatch_value", compact_str::format_compact!("{rewatch_value}")));
        }

        if let Some(tags) = self.tags {
            params.push(("tags", compact_str::format_compact!("{tags}")));
        }

        if let Some(comments) = self.comments {
            params.push(("comments", compact_str::format_compact!("{comments}")));
        }

        params
    }
}
