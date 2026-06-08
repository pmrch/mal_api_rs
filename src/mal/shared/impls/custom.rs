use super::traits::{HasTitles, Name};

impl super::models::SortOrder {
    #[must_use]
    pub const fn required_field(self) -> Option<compact_str::CompactString> {
        match self {
            Self::Title => None, // always present
            Self::MeanScore => Some(compact_str::CompactString::const_new("mean")),
            Self::StartDate => Some(compact_str::CompactString::const_new("start_date")),
            Self::Popularity => Some(compact_str::CompactString::const_new("popularity")),
            Self::Rank => Some(compact_str::CompactString::const_new("rank")),
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

impl std::fmt::Display for super::api::Ranking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stuff: Vec<String> = vec![];

        if let Some(prev) = self.previous_rank {
            stuff.push(format!("\nprevious rank: {prev}"));
        }

        stuff.push(format!("current rank: {}", self.rank));
        writeln!(f, "{}", stuff.join("\n"))
    }
}

impl super::api::AnimeNode {
    fn join_names<T>(input: &[T]) -> String
    where T: Name {
        input.iter().map(Name::name).collect::<Vec<compact_str::CompactString>>().join(", ")
    }

    #[must_use]
    pub fn display_some(&self) -> String {
        let t: &str = "average episode length: ";
        let t2: &str = "related anime: ";

        let fields: Vec<Option<compact_str::CompactString>> = vec![
            Some(compact_str::format_compact!("  id: {}", self.id)),
            Some(compact_str::format_compact!("title: {}", self.title)),
            self.mean.map(|m| compact_str::format_compact!("mean: {m:.1}")),
            self.nsfw.as_ref().map(|n| compact_str::format_compact!("nsfw: {n}")),
            self.num_episodes.map(|e| compact_str::format_compact!("episodes: {e}")),
            self.num_favorites.map(|nfavs| compact_str::format_compact!("favorited: {nfavs}")),
            self.num_list_users.map(|nlu| compact_str::format_compact!("users listed: {nlu}")),
            self.num_scoring_users.map(|nsu| compact_str::format_compact!("users scored: {nsu}")),
            self.media_type.as_ref().map(|mt| compact_str::format_compact!("media_type: {mt}")),
            self.popularity.map(|pop| compact_str::format_compact!("popularity: {pop}")),
            self.main_picture.as_ref().map(|mp| compact_str::format_compact!("main cover art: {mp}")),
            self.alternative_titles.as_ref().map(|at| compact_str::format_compact!("alternative titles: {at}")),
            self.broadcast.as_ref().map(|b| compact_str::format_compact!("broadcast: {b}")),
            self.created_at.as_ref().map(|cat| compact_str::format_compact!("created at: {cat}")),
            self.updated_at.as_ref().map(|ua| compact_str::format_compact!("updated at: {ua}")),
            self.start_date.as_ref().map(|sdate| compact_str::format_compact!("started at: {sdate}")),
            self.end_date.as_ref().map(|edate| compact_str::format_compact!("ended at: {edate}")),
            self.avg_ep_len.map(|v| compact_str::format_compact!("{t}{} minutes", v / 60)),
            self.genres.as_ref().map(|gens| compact_str::format_compact!("genres: [{}]", Self::join_names(gens))),
            self.rank.map(|rank| compact_str::format_compact!("ranking: #{rank}")),
            self.start_season.as_ref().map(|ss| compact_str::format_compact!("season: {} {}", ss.year, ss.season)),
            self.synopsis.as_ref().map(|syn| compact_str::format_compact!("synopsis: {syn}")),
            self.source.as_ref().map(|src| compact_str::format_compact!("source material: {src}")),
            self.studio.as_ref().map(|std| compact_str::format_compact!("studio: {}", std.name)),
            self.background.as_ref().map(|bg| compact_str::format_compact!("background story: {bg}")),
            self.related_anime.as_ref().map(|rel| compact_str::format_compact!("{t2}{}", Self::join_names(rel))),
            self.rating.as_ref().map(|rat| compact_str::format_compact!("rating: {rat}")),
        ];

        fields.into_iter().flatten().collect::<Vec<compact_str::CompactString>>().join("\n    ")
    }
}

impl AsRef<str> for super::api::Nsfw {
    fn as_ref(&self) -> &str {
        match self {
            Self::Black => "black",
            Self::Gray => "gray",
            Self::White => "white",
            Self::Unknown => "unknown",
        }
    }
}

impl AsRef<str> for super::api::AnimeRankingType {
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

impl HasTitles for super::api::AnimeNode {
    fn title(&self) -> &str { &self.title }
    fn alternative_titles(&self) -> Option<&crate::prelude::AlternativeTitles> { self.alternative_titles.as_ref() }
}
