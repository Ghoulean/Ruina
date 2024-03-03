use crate::models::Autocomplete;
use crate::models::DisambiguationPage;
use crate::models::TypedId;
use crate::models::PageType::AbnoPageId;
use crate::models::PageType::BattleSymbolId;
use crate::models::PageType::CombatPageId;
use crate::models::PageType::KeyPageId;
use crate::models::PageType::PassiveId;

pub mod models;

include!(concat!(env!("OUT_DIR"), "/out.rs"));


// static INVERSE_CARD_INDEX: phf::Map<&'static str, &[TypedId<'_>]> = ::phf::Map
// static DISAMBIGUATION_PAGES_KOREAN: phf::Map<&'static str, DisambiguationPage> = ::phf::Map
// static DISAMBIGUATION_MAP_KOREAN: phf::Map<&'static str, Autocomplete> = ::phf::Map
// static DISAMBIGUATION_PAGES_ENGLISH: phf::Map<&'static str, DisambiguationPage> = ::phf::Map
// static DISAMBIGUATION_MAP_ENGLISH: phf::Map<&'static str, Autocomplete> = ::phf::Map
// static DISAMBIGUATION_PAGES_JAPANESE: phf::Map<&'static str, DisambiguationPage> = ::phf::Map
// static DISAMBIGUATION_MAP_JAPANESE: phf::Map<&'static str, Autocomplete> = ::phf::Map
// static DISAMBIGUATION_PAGES_CHINESE: phf::Map<&'static str, DisambiguationPage> = ::phf::Map
// static DISAMBIGUATION_MAP_CHINESE: phf::Map<&'static str, Autocomplete> = ::phf::Map
// static DISAMBIGUATION_PAGES_TRADITIONALCHINESE: phf::Map<&'static str, DisambiguationPage> = ::phf::Map
// static DISAMBIGUATION_MAP_TRADITIONALCHINESE: phf::Map<&'static str, Autocomplete> = ::phf::Map
 