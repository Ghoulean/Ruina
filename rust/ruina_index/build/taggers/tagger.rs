#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Tag(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypedId(pub PageType, pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PageType {
    AbnoPageId,
    BattleSymbolId,
    CombatPageId,
    KeyPageId,
    PassiveId,
}

pub trait Tagger {
    fn get_typed_id(&self) -> TypedId;
    fn generate_tags(&self) -> Vec<Tag>;
}
