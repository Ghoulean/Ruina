use ruina_common::game_objects::combat_page::CombatPage;
use ruina_reparser::get_combat_page_locales_by_id;

use super::tagger::Tagger;

const KEY_PREFIX: &str = "c#";
const DEFAULT_COMBAT_PAGE_TAGS: [&str; 2] = ["combat", "page"];

impl Tagger for CombatPage<'_> {
    fn generate_tag_key(&self) -> String {
        format!("{}{}", KEY_PREFIX, self.id)
    }

    fn generate_tags(&self) -> Vec<String> {
        let default_tags = DEFAULT_COMBAT_PAGE_TAGS.to_vec();
        get_combat_page_locales_by_id(self.id)
            .values()
            .flat_map(|combat_page_locale| vec![combat_page_locale.name])
            .chain(default_tags)
            .map(String::from)
            .collect()
    }
}
