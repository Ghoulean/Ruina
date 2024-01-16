use ruina_common::game_objects::key_page::KeyPage;
use ruina_reparser::get_key_page_locales_by_id;

use super::tagger::Tagger;

const KEY_PREFIX: &str = "k#";
const DEFAULT_KEY_PAGE_TAGS: [&str; 2] = ["key", "page"];

impl Tagger for KeyPage<'_> {
    fn generate_tag_key(&self) -> String {
        format!("{}{}", KEY_PREFIX, self.id)
    }

    fn generate_tags(&self) -> Vec<String> {
        let default_tags = DEFAULT_KEY_PAGE_TAGS.to_vec();
        get_key_page_locales_by_id(self.id)
            .values()
            .flat_map(|key_page_locale| vec![key_page_locale.name])
            .chain(default_tags)
            .map(String::from)
            .collect()
    }
}
