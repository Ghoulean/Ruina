use ruina_common::game_objects::key_page::KeyPage;
use ruina_reparser::get_key_page_locales_by_id;

use super::tagger::PageType;
use super::tagger::Tag;
use super::tagger::Tagger;
use super::tagger::TypedId;

const DEFAULT_KEY_PAGE_TAGS: [&str; 2] = ["key", "page"];

impl Tagger for KeyPage<'_> {
    fn get_typed_id(&self) -> TypedId {
        TypedId(PageType::KeyPageId, String::from(self.id))
    }

    fn generate_tags(&self) -> Vec<Tag> {
        let default_tags = DEFAULT_KEY_PAGE_TAGS.to_vec();
        get_key_page_locales_by_id(self.id)
            .values()
            .flat_map(|key_page_locale| vec![key_page_locale.name])
            .chain(default_tags)
            .map(String::from)
            .map(Tag)
            .collect()
    }
}
