use ruina_common::game_objects::battle_symbol::BattleSymbol;
use ruina_reparser::get_battle_symbol_locales_by_internal_name;

use super::tagger::Tagger;

const KEY_PREFIX: &str = "b#";
const DEFAULT_BATTLE_SYMBOL_TAGS: [&str; 2] = ["battle", "symbol"];

impl Tagger for BattleSymbol<'_> {
    fn generate_tag_key(&self) -> String {
        format!("{}{}", KEY_PREFIX, self.id)
    }

    fn generate_tags(&self) -> Vec<String> {
        let default_tags = DEFAULT_BATTLE_SYMBOL_TAGS.to_vec();
        get_battle_symbol_locales_by_internal_name(self.internal_name)
            .values()
            .flat_map(|battle_symbol_locale| {
                vec![battle_symbol_locale.prefix, battle_symbol_locale.postfix]
            })
            .chain(default_tags)
            .map(String::from)
            .collect()
    }
}
