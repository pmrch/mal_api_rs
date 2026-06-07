impl crate::mal::shared::traits::HasTitles for super::models::MangaNode {
    fn title(&self) -> &str { &self.title }
    fn alternative_titles(&self) -> &Option<crate::prelude::AlternativeTitles> { &self.alternative_titles }
}
