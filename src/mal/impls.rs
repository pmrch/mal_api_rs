trait Name {
    fn name(&self) -> &str;
}

impl Name for super::models::Genre {
    fn name(&self) -> &str { self.name.as_str() }
}

impl Name for super::models::Anime {
    fn name(&self) -> &str { &self.node.title }
}

impl std::fmt::Display for super::models::AnimeRankingType {
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

impl AsRef<str> for super::models::AnimeRankingType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Airing => "airing",
            Self::All => "all",
            Self::ByPopularity => "bypopularity",
            Self::Favorite => "favorite",
            Self::Movie => "movie",
            Self::Ova => "ova",
            Self::Special => "special",
            Self::Tv => "tv",
            Self::Upcoming => "upcoming",
            Self::Unknown => "",
        }
    }
}

impl super::models::SortOrder {
    #[must_use]
    pub const fn required_field(self) -> Option<&'static str> {
        match self {
            Self::Title => None, // always present
            Self::MeanScore => Some("mean"),
            Self::StartDate => Some("start_date"),
            Self::Popularity => Some("popularity"),
            Self::Rank => Some("rank"),
        }
    }
}

impl super::models::NumEps {
    #[must_use]
    pub fn matches(self, episodes: u32) -> bool {
        match self {
            Self::LessThan(num_eps) => episodes < num_eps,
            Self::MoreThan(num_eps) => episodes > num_eps,
            Self::Exactly(num_eps) => episodes == num_eps,
            Self::Between(a, b) => {
                let min: u32 = a.min(b);
                let max: u32 = a.max(b);
                episodes >= min && episodes <= max
            }
        }
    }
}

impl super::models::EpLengthMins {
    #[must_use]
    pub fn matches(self, episode_len_secs: u32) -> bool {
        let ep_len_secs: f64 = f64::from(episode_len_secs);

        match self {
            Self::ShorterThan(num_mins) => f64::from(num_mins * 60) < ep_len_secs,
            Self::LongerThan(num_mins) => f64::from(num_mins * 60) > ep_len_secs,
            Self::Between(a, b) => {
                let min: f64 = f64::from(a.min(b));
                let max: f64 = f64::from(a.max(b));
                ep_len_secs >= min * 60f64 && ep_len_secs <= max * 60f64
            }
        }
    }
}

impl std::fmt::Display for super::models::AlternativeTitles {
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

impl std::fmt::Display for super::models::Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CurrentlyAiring => write!(f, "Currently airing"),
            Self::FinishedAiring => write!(f, "Finished airing"),
            Self::NotYetAired => write!(f, "Not yet aired"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::fmt::Display for super::models::MainPicture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = vec![format!("\n\t{}", &self.large), format!("medium: {}", &self.medium)];
        write!(f, "{}", parts.join("\n\t"))
    }
}

impl std::fmt::Display for super::models::Broadcast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = vec![format!("every {}", self.day_of_the_week), format!("at {}", self.start_time)];
        write!(f, "{}", parts.join(" "))
    }
}

impl std::fmt::Display for super::models::Genre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.name) }
}

impl std::fmt::Display for super::models::SeasonEnum {
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

impl super::models::AnimeNode {
    fn join_names<T>(input: &[T]) -> String
    where T: Name {
        input.iter().map(Name::name).collect::<Vec<&str>>().join(", ")
    }

    fn display_some(&self) -> String {
        let t: &str = "average episode length: ";
        let t2: &str = "related anime: ";

        let fields: Vec<Option<String>> = vec![
            Some(format!("  id: {}", self.id)),
            Some(format!("title: {}", self.title)),
            self.mean.map(|m| format!("mean: {m:.1}")),
            self.nsfw.as_ref().map(|n| format!("nsfw: {n}")),
            self.num_episodes.map(|e| format!("episodes: {e}")),
            self.num_favorites.map(|nfavs| format!("favorited: {nfavs}")),
            self.num_list_users.map(|nlu| format!("users listed: {nlu}")),
            self.num_scoring_users.map(|nsu| format!("users scored: {nsu}")),
            self.media_type.as_ref().map(|mt| format!("media_type: {mt}")),
            self.popularity.map(|pop| format!("popularity: {pop}")),
            self.main_picture.as_ref().map(|mp| format!("main cover art: {mp}")),
            self.alternative_titles.as_ref().map(|at| format!("alternative titles: {at}")),
            self.broadcast.as_ref().map(|b| format!("broadcast: {b}")),
            self.created_at.as_ref().map(|cat| format!("created at: {cat}")),
            self.updated_at.as_ref().map(|ua| format!("updated at: {ua}")),
            self.start_date.as_ref().map(|sdate| format!("started ad: {sdate}")),
            self.end_date.as_ref().map(|edate| format!("ended at: {edate}")),
            self.avg_ep_len.map(|v| format!("{t}{} minutes", v / 60)),
            self.genres.as_ref().map(|gens| format!("genres: [{}]", Self::join_names(gens))),
            self.rank.map(|rank| format!("ranking: #{rank}")),
            self.start_season.as_ref().map(|ss| format!("season: {} {}", ss.year, ss.season)),
            self.synopsis.as_ref().map(|syn| format!("synopsis: {syn}")),
            self.source.as_ref().map(|src| format!("source material: {src}")),
            self.studio.as_ref().map(|std| format!("studio: {}", std.name)),
            self.background.as_ref().map(|bg| format!("background story: {bg}")),
            self.related_anime.as_ref().map(|rel| format!("{t2}{}", Self::join_names(rel))),
            self.rating.as_ref().map(|rat| format!("rating: {rat}")),
        ];

        fields.into_iter().flatten().collect::<Vec<String>>().join("\n    ")
    }
}

impl std::fmt::Display for super::models::AnimeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "AnimeNode {{\n  {}\n}}", self.display_some()) }
}
