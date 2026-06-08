use super::{CompactString, HasTitles, Name};

impl HasTitles for super::models::MangaNode {
    fn title(&self) -> &str { &self.title }
    fn alternative_titles(&self) -> Option<&crate::prelude::AlternativeTitles> { self.alternative_titles.as_ref() }
}

impl HasTitles for super::models::Manga {
    fn title(&self) -> &str { &self.node.title }
    fn alternative_titles(&self) -> Option<&crate::prelude::AlternativeTitles> { self.node.alternative_titles.as_ref() }
}

impl Name for super::models::MangaAuthor {
    fn name(&self) -> CompactString { self.name().unwrap().into() }
}

impl Name for super::models::MangaRelatedAnime {
    fn name(&self) -> CompactString { self.node.title.as_str().into() }
}

impl Name for super::models::MangaRelationManga {
    fn name(&self) -> CompactString { self.node.title.clone() }
}

impl Name for super::models::MangaRecommendation {
    fn name(&self) -> CompactString { self.node.title.clone() }
}

impl Name for super::models::Serialization {
    fn name(&self) -> CompactString { self.node.name.clone() }
}
