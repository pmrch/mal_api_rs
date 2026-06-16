use std::fmt;

use super::{CompactString, HasTitles};

// -----------------------------------------------------------------------------
// Optional – a tiny enum formatter (if you need it elsewhere)
impl fmt::Display for super::models::RelationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &str = match self {
            Self::Sequel => "sequel",
            Self::Prequel => "prequel",
            Self::AlternativeSetting => "alternative_setting",
            Self::AlternativeVersion => "alternative_version",
            Self::SideStory => "side_story",
            Self::ParentStory => "parent_story",
            Self::Summary => "summary",
            Self::FullStory => "full_story",
        };
        write!(f, "{s}")
    }
}

impl std::fmt::Display for super::models::MangaRankingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Bypopularity => write!(f, "bypopularity"),
            Self::Doujin => write!(f, "doujin"),
            Self::Favorite => write!(f, "favorite"),
            Self::Manga => write!(f, "manga"),
            Self::Manhua => write!(f, "manhua"),
            Self::Manhwa => write!(f, "manhwa"),
            Self::Novels => write!(f, "novels"),
            Self::Oneshots => write!(f, "oneshots"),
        }
    }
}

/// Human‑readable enum – we make sure it has a nice string form first.
impl fmt::Display for super::models::MangaListStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &str = match self {
            Self::Reading => "reading",
            Self::Completed => "completed",
            Self::OnHold => "on hold",
            Self::Dropped => "dropped",
            Self::PlanToRead => "plan to read",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for super::models::MangaListStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Build up each non‑empty field.
        let mut parts: Vec<String> = Vec::with_capacity(15);

        parts.push(format!("status: {}", self.status));
        parts.push(format!("score: {}", self.score));
        parts.push(format!("volumes read: {}", self.num_volumes_read));
        parts.push(format!("chapters read: {}", self.num_chapters_read));
        parts.push(format!("rereading: {}", self.is_rereading));

        if let Some(d) = &self.start_date {
            parts.push(format!("start date: {}", d.format("%Y-%m-%d")));
        }
        if let Some(d) = &self.finish_date {
            parts.push(format!("finish date: {}", d.format("%Y-%m-%d")));
        }

        parts.push(format!("priority: {}", self.priority));
        parts.push(format!("times reread: {}", self.num_times_reread));
        parts.push(format!("reread value: {}", self.reread_value));

        if !self.tags.is_empty() {
            let tags = self.tags.iter().map(CompactString::as_str).collect::<Vec<&str>>().join(", ");
            parts.push(format!("tags: [{tags}]"));
        }

        if let Some(c) = &self.comments {
            parts.push(format!("comments: {c}"));
        }

        // Updated at is mandatory – format it in ISO‑8601 UTC.
        parts.push(format!(
            "updated at: {}",
            self.updated_at.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
        ));

        write!(f, "{}", parts.join("\n    "))
    }
}

impl fmt::Display for super::models::MangaAuthor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fields: Vec<String> = Vec::with_capacity(3);
        fields.push(format!("\n{}", self.node.id));

        if let Some(first) = &self.node.first_name {
            fields.push(first.to_string());
        }

        if let Some(last) = &self.node.last_name {
            fields.push(last.to_string());
        }

        write!(f, "{}", fields.join("\n"))
    }
}

impl fmt::Display for super::models::MangaPictures {
    /// Human‑readable representation of a `MangaPictures` value.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fields: Vec<String> = Vec::with_capacity(2);

        fields.push(format!("medium: {}", self.medium));
        if let Some(ref large) = self.large {
            fields.push(format!("large: {large}"));
        }

        // Join them with a comma‑space and write to the formatter.
        write!(f, "{}", fields.join(", "))
    }
}

// -----------------------------------------------------------------------------
// Optional – you can also give a human‑friendly view for the ranking query
// data.
// -----------------------------------------------------------------------------
impl fmt::Display for super::models::MangaRankingQueryData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.node, // MangaNode's Display impl
            self.ranking
        )
    }
}

// -----------------------------------------------------------------------------
// 4️⃣  MangaRelationManga (node + relation type)
impl fmt::Display for super::models::MangaRelationManga {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Show only the most relevant parts of the node
        write!(
            f,
            "{}\n    Relation: {}",
            self.node.title(), // use the title helper from MangaNode
            self.relation_type_formatted
        )
    }
}

// -----------------------------------------------------------------------------
// 5️⃣  MangaRelatedAnime (node + relation type)
impl fmt::Display for super::models::MangaRelatedAnime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n    Relation: {}",
            self.node.title(), // AnimeNode's `title()` helper
            self.relation_type_formatted
        )
    }
}

// --------------------------------------------------------
// The actual `Display` implementation
// --------------------------------------------------------
impl fmt::Display for super::models::MangaNode {
    /// Render the node exactly like `display_some()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // `write!` writes the returned String directly into the formatter.
        write!(f, "{}", self.display_some())
    }
}

// -----------------------------------------------------------------------------
// 1️⃣  SerializationNode (id + name)
impl fmt::Display for super::models::SerializationNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // A very short, readable representation
        write!(f, "Serialization {} ({})", self.name, self.id)
    }
}

// -----------------------------------------------------------------------------
// 3️⃣  MangaRecommendation (node + number of recommendations)
impl fmt::Display for super::models::MangaRecommendation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n    Recommendations: {}", self.node, self.num_recommendations)
    }
}

// -----------------------------------------------------------------------------
// 2️⃣  Serialization (node + role)
impl fmt::Display for super::models::Serialization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}  - Role: {}", self.node, self.role) }
}

impl fmt::Display for super::models::MangaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_ref()) }
}

impl fmt::Display for super::models::MangaStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_ref()) }
}
