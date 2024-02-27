use crate::models::Autocomplete;
use crate::models::DisambiguationPage;
use crate::models::TypedId;
use crate::models::PageType::AbnoPageId;
use crate::models::PageType::BattleSymbolId;
use crate::models::PageType::CombatPageId;
use crate::models::PageType::KeyPageId;
use crate::models::PageType::PassiveId;

pub mod models;


/***
 * Exposes the following as public:
 *
 */
include!(concat!(env!("OUT_DIR"), "/out.rs"));

// #[path = "tagger.rs"]
// pub mod taggers;