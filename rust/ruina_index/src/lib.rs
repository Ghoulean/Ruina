pub mod models;

use std::collections::HashSet;

use crate::models::Autocomplete;
use crate::models::DisambiguationPage;
use crate::models::TypedId;
use crate::models::PageType::AbnoPageId;
use crate::models::PageType::BattleSymbolId;
use crate::models::PageType::CombatPageId;
use crate::models::PageType::KeyPageId;
use crate::models::PageType::PassiveId;
pub use ruina_index_analyzer::analyze;
use ruina_common::localizations::common::Locale;

include!(concat!(env!("OUT_DIR"), "/out.rs"));

pub fn get_typed_ids(index: &str) -> Option<HashSet<&TypedId<'static>>> {
    INVERSE_CARD_INDEX.get(index).and_then(|x| Some(HashSet::from_iter(x.into_iter())))
}

pub fn get_disambiguation_page(display_name: &str, locale: Locale) -> Option<&DisambiguationPage> {
    match locale {
        Locale::Korean => DISAMBIGUATION_PAGES_KOREAN.get(display_name),
        Locale::English => DISAMBIGUATION_PAGES_ENGLISH.get(display_name),
        Locale::Japanese => DISAMBIGUATION_PAGES_JAPANESE.get(display_name),
        Locale::Chinese => DISAMBIGUATION_PAGES_CHINESE.get(display_name),
        Locale::TraditionalChinese => DISAMBIGUATION_PAGES_TRADITIONALCHINESE.get(display_name),
    }
}

pub fn get_autocomplete_entry<'a>(typed_id: &'a TypedId<'static>, locale: Locale) -> Option<&'a Autocomplete<'static>> {
    let serialized_typed_id = format!("{:?}{:}", typed_id.0, typed_id.1);
    match locale {
        Locale::Korean => DISAMBIGUATION_MAP_KOREAN.get(&serialized_typed_id),
        Locale::English => DISAMBIGUATION_MAP_ENGLISH.get(&serialized_typed_id),
        Locale::Japanese => DISAMBIGUATION_MAP_JAPANESE.get(&serialized_typed_id),
        Locale::Chinese => DISAMBIGUATION_MAP_CHINESE.get(&serialized_typed_id),
        Locale::TraditionalChinese => DISAMBIGUATION_MAP_TRADITIONALCHINESE.get(&serialized_typed_id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_typed_ids_sanity_check() {
        let degraded_pillar = TypedId(CombatPageId, "607204");

        let hashset = get_typed_ids("pillar");
        assert!(hashset.is_some());
        assert!(hashset.unwrap().get(&degraded_pillar).is_some());
    }

    #[test]
    fn get_disambiguation_page_sanity_check() {
        assert!(get_disambiguation_page("Prepared Mind", Locale::English).is_some());
    }

    #[test]
    fn get_autocomplete_entry_sanity_check() {
        let degraded_pillar = TypedId(CombatPageId, "607204");

        assert!(get_autocomplete_entry(&degraded_pillar, Locale::English).is_some());
    }

}