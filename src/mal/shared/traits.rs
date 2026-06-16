use compact_str::CompactString;

use super::api::{AlternativeTitles, AnimeNode};
#[cfg(feature = "manga")]
use crate::mal::manga::api::MangaNode;

pub trait HasNode {
    fn node(&self) -> &AnimeNode;
}

pub trait Name {
    fn name(&self) -> CompactString;
}

#[cfg(feature = "manga")]
pub trait MangaHasNode {
    fn node(&self) -> &MangaNode;
}

pub trait HasTitles {
    fn title(&self) -> &str;
    fn alternative_titles(&self) -> Option<&AlternativeTitles>;
}
