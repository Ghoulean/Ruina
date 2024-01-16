use ruina_common::game_objects::abno_page::AbnoPage;
use ruina_reparser::get_abno_page_locales_by_internal_name;

use super::tagger::Tagger;

const KEY_PREFIX: &str = "a#";
const DEFAULT_ABNO_TAGS: [&str; 1] = ["abno"];

impl Tagger for AbnoPage<'_> {
    fn generate_tag_key(&self) -> String {
        format!("{}{}", KEY_PREFIX, self.id)
    }

    fn generate_tags(&self) -> Vec<String> {
        let default_tags = DEFAULT_ABNO_TAGS.to_vec();
        get_abno_page_locales_by_internal_name(self.internal_name)
            .values()
            .flat_map(|abno_page_locale| {
                vec![abno_page_locale.card_name, abno_page_locale.abnormality]
            })
            .chain(default_tags)
            .map(String::from)
            .collect()
    }
}
