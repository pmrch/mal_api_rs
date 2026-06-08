use std::fmt::Write;

use super::{CompactString, Name};

impl AsRef<str> for super::models::MangaType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Doujinshi => "doujinshi",
            Self::Manga => "manga",
            Self::Manhua => "manhua",
            Self::Manhwa => "manhwa",
            Self::Novel => "novel",
            Self::Oel => "oel",
            Self::OneShot => "one_shot",
            Self::Unknown => "unknown",
        }
    }
}

impl AsRef<str> for super::models::MangaStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::CurrentlyPublishing => "currently publishing",
            Self::Finished => "finished",
            Self::NoyYetPublished => "not yet published",
            Self::Unknown => "unknown",
        }
    }
}

impl super::models::MangaAuthor {
    pub fn name(&self) -> std::result::Result<String, std::fmt::Error> {
        let mut name_string = String::new();
        if let Some(first) = &self.node.first_name {
            write!(name_string, "{first}")?;
        }

        if let Some(last) = &self.node.last_name {
            write!(name_string, " {last}")?;
        }

        Ok(name_string)
    }
}

// --------------------------------------------------------
// Helper & pretty‑printing logic
// --------------------------------------------------------
impl super::models::MangaNode {
    /// Turn a slice of items that implement `Name` into a comma‑separated
    /// string.
    fn join_names<T>(input: &[T]) -> String
    where T: Name {
        input.iter().map(Name::name).collect::<Vec<CompactString>>().join(", ")
    }

    /// Return the human‑readable representation that you already use for
    /// `AnimeNode`.
    #[must_use]
    pub fn display_some(&self) -> String {
        let fields: Vec<Option<CompactString>> = vec![
            Some(CompactString::from(format!("id: {}", self.id))),
            Some(CompactString::from(format!("title: {}", self.title))),
            // Optional scalar / enum values
            self.main_picture.as_ref().map(|mp| CompactString::from(format!("main cover art: {mp}"))),
            self.alternative_titles.as_ref().map(|at| CompactString::from(format!("alternative titles: {at}"))),
            self.start_date.as_ref().map(|d| CompactString::from(format!("started at: {}", d.format("%Y-%m-%d")))),
            self.end_date.as_ref().map(|d| CompactString::from(format!("ended at: {}", d.format("%Y-%m-%d")))),
            self.synopsis.as_ref().map(|s| CompactString::from(format!("synopsis: {s}"))),
            self.mean.map(|m| CompactString::from(format!("mean: {:.1}", m.into_inner()))),
            self.rank.map(|r| CompactString::from(format!("ranking: #{r}"))),
            self.popularity.map(|p| CompactString::from(format!("popularity: {p}"))),
            self.num_list_users.map(|nlu| CompactString::from(format!("users listed: {nlu}"))),
            self.num_scoring_users.map(|nsu| CompactString::from(format!("users scored: {nsu}"))),
            self.nsfw.as_ref().map(|n| CompactString::from(format!("nsfw: {n}"))),
            self.genres.as_ref().map(|gs| CompactString::from(format!("genres: [{}]", Self::join_names(gs)))),
            self.created_at.as_ref().map(|c| CompactString::from(format!("created at: {}", c.to_rfc3339()))),
            self.updated_at.as_ref().map(|u| CompactString::from(format!("updated at: {}", u.to_rfc3339()))),
            // More enums / optionals
            self.media_type.as_ref().map(|mt| CompactString::from(format!("media_type: {mt}"))),
            self.status.as_ref().map(|st| CompactString::from(format!("status: {st}"))),
            self.my_list_status.as_ref().map(|mls| CompactString::from(format!("my list status: {mls}"))),
            // Volumes / chapters
            self.num_volumes.map(|nv| CompactString::from(format!("volumes: {nv}"))),
            self.num_chapters.map(|nc| CompactString::from(format!("chapters: {nc}"))),
            // Complex collections
            self.authors.as_ref().map(|a| CompactString::from(format!("authors: [{}]", Self::join_names(a)))),
            self.ranking.as_ref().map(|rnk| CompactString::from(format!("ranking info: {rnk}"))),
            self.pictures.as_ref().map(|p| CompactString::from(format!("pictures: {p}"))),
            // Optional strings
            self.background.as_ref().map(|bg| CompactString::from(format!("background story: {bg}"))),
            // Related stuff (all use `join_names`)
            self.related_anime.as_ref().map(|ra| CompactString::from(format!("related anime: [{}]", Self::join_names(ra)))),
            self.related_manga.as_ref().map(|rm| CompactString::from(format!("related manga: [{}]", Self::join_names(rm)))),
            self.recommendations.as_ref().map(|rec| CompactString::from(format!("recommendations: [{}]", Self::join_names(rec)))),
            self.serialization.as_ref().map(|ser| CompactString::from(format!("serialization: [{}]", Self::join_names(ser)))),
        ];

        fields.into_iter().flatten().collect::<Vec<CompactString>>().join("\n")
    }
}
