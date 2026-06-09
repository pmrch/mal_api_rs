use super::CompactString;
use super::models::MangaListStatusEnum;

#[derive(Default)]
/// Builder for updating a manga entry on the authenticated user's MAL list.
/// Only fields that are explicitly set will be included in the update request.
/// Unset fields remain unchanged on MAL's side.
pub struct MangaUpdateBuilder {
    status:            Option<MangaListStatusEnum>,
    is_rereading:      Option<bool>,
    score:             Option<u8>, // 0‑10
    num_volumes_read:  Option<u32>,
    num_chapters_read: Option<u32>,
    priority:          Option<u8>, // 0‑2
    num_times_reread:  Option<u8>, // 0‑5
    reread_value:      Option<u8>, // 0‑5
    tags:              Option<CompactString>,
    comments:          Option<CompactString>,
}

impl MangaUpdateBuilder {
    pub fn new() -> Self { Self::default() }

    /// Set the list status for this manga.
    pub const fn status(mut self, status: MangaListStatusEnum) -> Self {
        self.status = Some(status);
        self
    }

    /// Set whether the user is currently rereading the manga.
    pub const fn rereading(mut self, rereading: bool) -> Self {
        self.is_rereading = Some(rereading);
        self
    }

    /// Set the user's score for this manga.  Clamped to `0‑10`.
    pub fn score(mut self, score: u8) -> Self {
        self.score = Some(score.min(10));
        self
    }

    /// Set how many volumes have been read.
    pub const fn volumes_read(mut self, vol_read: u32) -> Self {
        self.num_volumes_read = Some(vol_read);
        self
    }

    /// Set how many chapters have been read.
    pub const fn chapters_read(mut self, chap_read: u32) -> Self {
        self.num_chapters_read = Some(chap_read);
        self
    }

    /// Set the priority of this manga entry.  Clamped to `0‑2`.
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority.min(2));
        self
    }

    /// Set how many times the user has reread the manga.
    pub const fn times_reread(mut self, num_reread: u8) -> Self {
        // `num_times_reread` is a u8 in your original struct – keep it that way.
        self.num_times_reread = Some(num_reread);
        self
    }

    /// Set the reread value for this manga.  Clamped to `0‑5`.
    pub fn reread_value(mut self, reread_value: u8) -> Self {
        self.reread_value = Some(reread_value.min(5));
        self
    }

    /// User‑supplied tags.
    pub fn tags(mut self, tags: CompactString) -> Self {
        self.tags = Some(tags);
        self
    }

    /// User‑supplied comments.
    pub fn comments(mut self, comments: CompactString) -> Self {
        self.comments = Some(comments);
        self
    }

    /// Consume the builder and return a vector of key/value pairs ready to
    /// be encoded into the query string or JSON body for MAL.
    pub(crate) fn into_params(self) -> Vec<(&'static str, compact_str::CompactString)> {
        let mut params: Vec<(&str, compact_str::CompactString)> = vec![];

        if let Some(status) = self.status {
            params.push(("status", compact_str::format_compact!("{status}")));
        }
        if let Some(is_rereading) = self.is_rereading {
            params.push(("is_rereading", compact_str::format_compact!("{is_rereading}")));
        }
        if let Some(score) = self.score {
            params.push(("score", compact_str::format_compact!("{score}")));
        }
        if let Some(num_volumes_read) = self.num_volumes_read {
            params.push(("num_volumes_read", compact_str::format_compact!("{num_volumes_read}")));
        }
        if let Some(num_chapters_read) = self.num_chapters_read {
            params.push(("num_chapters_read", compact_str::format_compact!("{num_chapters_read}")));
        }
        if let Some(priority) = self.priority {
            params.push(("priority", compact_str::format_compact!("{priority}")));
        }
        if let Some(num_times_reread) = self.num_times_reread {
            params.push(("num_times_reread", compact_str::format_compact!("{num_times_reread}")));
        }
        if let Some(reread_value) = self.reread_value {
            params.push(("reread_value", compact_str::format_compact!("{reread_value}")));
        }
        if let Some(tags) = self.tags {
            params.push(("tags", tags));
        }
        if let Some(comments) = self.comments {
            params.push(("comments", comments));
        }

        params
    }
}
