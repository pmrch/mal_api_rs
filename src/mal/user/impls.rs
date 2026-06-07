use std::fmt::Display;

impl super::Statistics {
    pub fn display_some(&self) -> String {
        let fields: Vec<compact_str::CompactString> = vec![
            compact_str::format_compact!("watching: {}", self.num_items_watching),
            compact_str::format_compact!("completed: {}", self.num_items_completed),
            compact_str::format_compact!("on hold: {}", self.num_items_on_hold),
            compact_str::format_compact!("dropped: {}", self.num_items_dropped),
            compact_str::format_compact!("plan to watch: {}", self.num_items_plan_to_watch),
            compact_str::format_compact!("total items: {}", self.num_items),
            compact_str::format_compact!("days watched: {:.1}", self.num_days_watched),
            compact_str::format_compact!("days watching: {:.1}", self.num_days_watching),
            compact_str::format_compact!("days completed: {:.1}", self.num_days_completed),
            compact_str::format_compact!("days on hold: {:.1}", self.num_days_on_hold),
            compact_str::format_compact!("days dropped: {:.1}", self.num_days_dropped),
            compact_str::format_compact!("total days: {:.1}", self.num_days),
            compact_str::format_compact!("episodes: {}", self.num_episodes),
            compact_str::format_compact!("rewatched: {}", self.num_times_rewatched),
            compact_str::format_compact!("mean score: {:.2}", self.mean_score),
        ];

        fields.into_iter().collect::<Vec<_>>().join("\n    ")
    }
}

impl super::UserInfo {
    pub fn display_some(&self) -> String {
        let fields: Vec<Option<compact_str::CompactString>> = vec![
            Some(compact_str::format_compact!("id: {}", self.id)),
            Some(compact_str::format_compact!("name: {}", self.name)),
            Some(compact_str::format_compact!("picture: {}", self.picture)),
            self.gender.as_ref().map(|g| compact_str::format_compact!("gender: {g}")),
            self.birthday.as_ref().map(|b| compact_str::format_compact!("birthday: {b}")),
            self.location.as_ref().map(|l| compact_str::format_compact!("location: {l}")),
            Some(compact_str::format_compact!("joined at: {}", self.joined_at)),
            self.anime_statistics.as_ref().map(|s| compact_str::format_compact!("anime statistics:\n    {}", s.display_some())),
            self.timezone.as_ref().map(|tz| compact_str::format_compact!("timezone: {tz}")),
            self.is_supporter.map(|s| compact_str::format_compact!("supporter: {s}")),
        ];

        fields.into_iter().flatten().collect::<Vec<_>>().join("\n    ")
    }
}

impl Display for super::Statistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { writeln!(f, "{}", self.display_some()) }
}

impl Display for super::UserInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { writeln!(f, "{}", self.display_some()) }
}
