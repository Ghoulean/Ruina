use ruina_common::game_objects::passive::Passive;
use ruina_reparser::get_passive_locales_by_id;

use super::tagger::Tagger;

const KEY_PREFIX: &str = "p#";
const DEFAULT_PASSIVE_TAGS: [&str; 1] = ["passive"];

impl Tagger for Passive<'_> {
    fn generate_tag_key(&self) -> String {
        format!("{}{}", KEY_PREFIX, self.id)
    }

    fn generate_tags(&self) -> Vec<String> {
        let default_tags = DEFAULT_PASSIVE_TAGS.to_vec();
        get_passive_locales_by_id(self.id)
            .values()
            .flat_map(|passive_locale| vec![passive_locale.name])
            .chain(default_tags)
            .map(String::from)
            .collect()
    }
}
