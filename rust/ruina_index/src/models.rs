pub struct Autocomplete<'a> {
    pub base: &'a str,
    pub disambiguator: Option<&'a str>
}

pub struct DisambiguationPage<'a> {
    pub id: &'a str,
    pub typed_ids: &'a [TypedId<'a>],
    pub default: Option<&'a str>
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypedId<'a>(pub PageType, pub &'a str);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PageType {
    AbnoPageId,
    BattleSymbolId,
    CombatPageId,
    KeyPageId,
    PassiveId,
}
