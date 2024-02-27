pub struct Autocomplete<'a> {
    pub base: &'a str,
    pub disambiguator: Option<TypedId<'a>>
}

pub struct DisambiguationPage<'a> {
    pub id: &'a str,
    pub typed_ids: &'a [TypedId<'a>],
    pub default: Option<&'a str>
}

pub struct TypedId<'a>(pub PageType, pub &'a str);

pub enum PageType {
    AbnoPageId,
    BattleSymbolId,
    CombatPageId,
    KeyPageId,
    PassiveId,
}
