use ruina_index::analyze;
use ruina_index::models::TypedId;
use ruina_index::INVERSE_CARD_INDEX;

pub fn get_typed_id(query: String) -> &'static TypedId<'static> {
    let tokens = analyze(&query);
    let typed_ids = tokens.iter().flat_map(|x| {
        INVERSE_CARD_INDEX.get(x.0.as_str())
    });
    todo!()
}

fn set_intersection() -> bool {
    todo!()
}
