impl std::fmt::Display for super::models::QuerySort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnimeId => write!(f, "anime_id"),
            Self::AnimeTitle => write!(f, "anime_title"),
            Self::AnimeStartDate => write!(f, "anime_start_date"),
            Self::ListScore => write!(f, "list_score"),
            Self::ListUpdatedAt => write!(f, "list_updated_at"),
        }
    }
}

impl std::fmt::Display for super::api::AnimeRankingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Airing => write!(f, "airing"),
            Self::All => write!(f, "all"),
            Self::ByPopularity => write!(f, "bypopularity"),
            Self::Favorite => write!(f, "favorite"),
            Self::Movie => write!(f, "movie"),
            Self::Ova => write!(f, "ova"),
            Self::Special => write!(f, "special"),
            Self::Tv => write!(f, "tv"),
            Self::Upcoming => write!(f, "upcoming"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::fmt::Display for super::api::ListStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Completed => write!(f, "completed"),
            Self::Dropped => write!(f, "dropped"),
            Self::OnHold => write!(f, "on_hold"),
            Self::PlanToWatch => write!(f, "plan_to_watch"),
            Self::Watching => write!(f, "watching"),
            Self::Unknown => {
                tracing::warn!("Defaulting to completed. Invalid ListStatusEnum was provided: {self}");
                write!(f, "completed")
            }
        }
    }
}

impl std::fmt::Display for super::api::AlternativeTitles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts: Vec<String> = vec![];

        if !self.en.is_empty() {
            parts.push(format!("\n\ten: {}", self.en));
        }

        if !self.ja.is_empty() {
            parts.push(format!("ja: {}", self.ja));
        }

        if !self.synonyms.is_empty() {
            parts.push(format!("synonyms: [{}]", self.synonyms.join(", ")));
        }

        if parts.is_empty() {
            write!(f, "AlternativeTitles {{ }}")
        } else {
            write!(f, "{}", parts.join("\n\t"))
        }
    }
}

impl std::fmt::Display for super::api::Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CurrentlyAiring => write!(f, "Currently airing"),
            Self::FinishedAiring => write!(f, "Finished airing"),
            Self::NotYetAired => write!(f, "Not yet aired"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::fmt::Display for super::api::MainPicture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = vec![format!("\n\t{}", &self.large), format!("medium: {}", &self.medium)];
        write!(f, "{}", parts.join("\n\t"))
    }
}

impl std::fmt::Display for super::api::Broadcast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = vec![format!("every {}", self.day_of_the_week), format!("at {}", self.start_time)];
        write!(f, "{}", parts.join(" "))
    }
}

impl std::fmt::Display for super::api::Genre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.name) }
}

impl std::fmt::Display for super::api::SeasonEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fall => write!(f, "fall"),
            Self::Spring => write!(f, "spring"),
            Self::Winter => write!(f, "winter"),
            Self::Summer => write!(f, "summer"),
            Self::Unknown => {
                tracing::warn!("Unknown season received, default to summer");
                write!(f, "summer")
            }
        }
    }
}

impl std::fmt::Display for super::api::AnimeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "AnimeNode {{\n  {}\n}}", self.display_some()) }
}
