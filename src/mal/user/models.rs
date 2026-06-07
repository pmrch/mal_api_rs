use super::{CompactString, DateTime, Deserialize, NaiveDate, OrderedFloat, Url, Utc};

#[derive(Debug, Clone, Deserialize)]
pub struct Statistics {
    pub num_items_watching:      u32,
    pub num_items_completed:     u32,
    pub num_items_on_hold:       u32,
    pub num_items_dropped:       u32,
    pub num_items_plan_to_watch: u32,
    pub num_items:               u32,
    pub num_days_watched:        OrderedFloat<f32>,
    pub num_days_watching:       OrderedFloat<f32>,
    pub num_days_completed:      OrderedFloat<f32>,
    pub num_days_on_hold:        OrderedFloat<f32>,
    pub num_days_dropped:        OrderedFloat<f32>,
    pub num_days:                OrderedFloat<f32>,
    pub num_episodes:            u32,
    pub num_times_rewatched:     u32,
    pub mean_score:              OrderedFloat<f32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    pub id:               i32,
    pub name:             CompactString,
    pub picture:          Url,
    pub gender:           Option<CompactString>,
    pub birthday:         Option<NaiveDate>,
    pub location:         Option<CompactString>,
    pub joined_at:        DateTime<Utc>,
    pub anime_statistics: Option<Statistics>,
    pub timezone:         Option<CompactString>,
    pub is_supporter:     Option<bool>,
}
