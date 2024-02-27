use std::collections::HashMap;
use std::string::ToString;

use ruina_common::localizations::common::Locale;
use ruina_reparser::ABNO_PAGES;
use ruina_reparser::{
    get_abno_page_locales_by_internal_name, get_battle_symbol_locales_by_internal_name,
    get_combat_page_locales_by_id, get_key_page_locales_by_id, get_passive_locales_by_id,
    BATTLE_SYMBOLS, COMBAT_PAGES, KEY_PAGES, PASSIVES,
};

use crate::autocomplete::differentiators::typed_id_disambiguator;
use crate::autocomplete::models::DisambiguatedAutocomplete;
use crate::taggers::tagger::Tagger;
use crate::taggers::tagger::TypedId;

use super::models::{AmbiguousAutocomplete, AmbiguousAutocompleteMap, DisambiguationDisplay, IncompleteAutocompleteMap};

pub fn generate_serialized_autocomplete_objs(locale: &Locale) -> String {
    let naive_map = generate_naive_autocomplete_map(locale);
    let disambiguation_pages = generate_serialized_disambiguation_page_array(&naive_map, locale);
    let mut editable_map = naive_map.clone();
    differentiate_by(&mut editable_map, typed_id_disambiguator);
    let disambiguation_map = generate_serialized_disambiguation_map(&editable_map, locale);
    
    [
        disambiguation_pages,
        disambiguation_map
    ].join("\n")
}

// todo: please generalize this. it hurts to look at.
fn generate_naive_autocomplete_map(locale: &Locale) -> IncompleteAutocompleteMap {
    let mut ret_val = IncompleteAutocompleteMap(HashMap::new());

    ABNO_PAGES
        .iter()
        .filter_map(|page| {
            get_abno_page_locales_by_internal_name(page.internal_name)
                .get(locale)
                .map(|x| (page.get_typed_id(), x.card_name))
        })
        .for_each(|(id, autocomplete)| {
            try_insert_incomplete_map(
                &mut ret_val,
                id,
                AmbiguousAutocomplete(String::from(autocomplete)),
            );
        });

    BATTLE_SYMBOLS
        .iter()
        .filter_map(|page| {
            get_battle_symbol_locales_by_internal_name(page.internal_name)
                .get(locale)
                .map(|x| (page.get_typed_id(), format!("{} {}", x.prefix, x.postfix)))
        })
        .for_each(|(id, autocomplete)| {
            try_insert_incomplete_map(
                &mut ret_val,
                id,
                AmbiguousAutocomplete(String::from(autocomplete)),
            );
        });

    COMBAT_PAGES
        .iter()
        .filter_map(|page| {
            get_combat_page_locales_by_id(page.id)
                .get(locale)
                .map(|x| (page.get_typed_id(), x.name))
        })
        .for_each(|(id, autocomplete)| {
            try_insert_incomplete_map(
                &mut ret_val,
                id,
                AmbiguousAutocomplete(String::from(autocomplete)),
            );
        });

    KEY_PAGES
        .iter()
        .filter_map(|page| {
            get_key_page_locales_by_id(page.id)
                .get(locale)
                .map(|x| (page.get_typed_id(), x.name))
        })
        .for_each(|(id, autocomplete)| {
            try_insert_incomplete_map(
                &mut ret_val,
                id,
                AmbiguousAutocomplete(String::from(autocomplete)),
            );
        });

    PASSIVES
        .iter()
        .filter_map(|page| {
            get_passive_locales_by_id(page.id)
                .get(locale)
                .map(|x| (page.get_typed_id(), x.name))
        })
        .for_each(|(id, autocomplete)| {
            try_insert_incomplete_map(
                &mut ret_val,
                id,
                AmbiguousAutocomplete(String::from(autocomplete)),
            );
        });

    ret_val
}

fn try_insert_incomplete_map(
    incomplete_map: &mut IncompleteAutocompleteMap,
    id: TypedId,
    autocomplete: AmbiguousAutocomplete,
) {
    let ambiguous_autocomplete_map = incomplete_map
        .0
        .entry(autocomplete.clone())
        .or_insert(AmbiguousAutocompleteMap(HashMap::new()));
    assert!(
        ambiguous_autocomplete_map
            .0
            .insert(
                id.clone(),
                DisambiguatedAutocomplete(autocomplete.clone(), None)
            )
            .is_none(),
        "duplicate entry detected! trying to insert: {id:?}->{autocomplete:?}"
    );
}

// todo: organize this function somewhere else?
// todo: subfunctions
fn generate_serialized_disambiguation_page_array(naive_map: &IncompleteAutocompleteMap, locale: &Locale) -> String {
    let mut builder = phf_codegen::Map::new();
    for (ambiguous_autocomplete, ambiguous_autocomplete_map) in &(naive_map.0) {
        if ambiguous_autocomplete_map.0.len() <= 1 {
            dbg!("autocomplete entry does not need disambiguation:", ambiguous_autocomplete);
            continue;
        }

        let base_autocomplete = ambiguous_autocomplete.0.clone();
        let associated_page_ids = ambiguous_autocomplete_map
            .0
            .keys()
            .map(|typed_id| format!("{:?}", typed_id))
            .collect::<Vec<String>>()
            .join(",");

        // todo: default pageId determination
        // serialized Option<&str>
        let default = "None";
        builder.entry(
            base_autocomplete.clone(),
            &format!(
                "DisambiguationPage {{
                id: r#\"{base_autocomplete}\"#,
                typed_ids: &[{associated_page_ids}],
                default: {default}
            }}"
            ),
        );
    }
    format!(
        "static DISAMBIGUATION_PAGES_{}: phf::Map<&'static str, DisambiguationPage> = {};",
        locale.to_string().to_ascii_uppercase(),
        builder.build()
    )
}

fn differentiate_by(
    incomplete_map: &mut IncompleteAutocompleteMap,
    predicate: fn(&TypedId) -> Option<DisambiguationDisplay>
) {
    incomplete_map.0.iter_mut()
        .for_each(|(_, map)|{
            let binding = map.clone();
            let unique = identify_unique(&binding, predicate);
            if unique.is_some() {
                add_disambiguation(
                    map,
                    unique.unwrap(),
                    predicate(&unique.unwrap()).unwrap()
                )
            }
        });
}

fn identify_unique(
    ambiguous_autocomplete_map: &AmbiguousAutocompleteMap,
    predicate: fn(&TypedId) -> Option<DisambiguationDisplay>
) -> Option<&TypedId> {
    let matches = ambiguous_autocomplete_map.0
        .keys()
        .filter(|x| predicate(x).is_some())
        .collect::<Vec<&TypedId>>();
    if matches.len() == 1 {
        return matches.first().copied();
    }
    None
}

fn add_disambiguation(
    ambiguous_autocomplete_map: &mut AmbiguousAutocompleteMap,
    typed_id: &TypedId,
    disambiguation_display: DisambiguationDisplay
) {
    let _ = ambiguous_autocomplete_map
        .0
        .get_mut(typed_id)
        .expect("typed_id not found")
        .1
        .insert(disambiguation_display.clone());
}

// todo: organize this function somewhere else?
// todo: subfunctions
fn generate_serialized_disambiguation_map(disambiguated_map: &IncompleteAutocompleteMap, locale: &Locale) -> String {
    let mut builder = phf_codegen::Map::new();

    disambiguated_map.0.values().into_iter().for_each(|inverse_map| {
        inverse_map.0.clone().into_iter().for_each(|(typed_id, disambiguated_autocomplete)| {
            let base = disambiguated_autocomplete.0.0;
            let disambiguator = serialize_option(disambiguated_autocomplete.1);

            builder.entry(
                format!("{:?}", typed_id), 
                &format!(
                    "Autocomplete {{
                        base: r#\"{base}\"#,
                        disambiguator: {disambiguator},
                    }}"
                )
            );
        });
    });

    format!(
        "static DISAMBIGUATION_MAP_{}: phf::Map<&'static str, Autocomplete> = {};",
        locale.to_string().to_ascii_uppercase(),
        builder.build()
    )
}

fn serialize_option(option: Option<DisambiguationDisplay>) -> String {
    if option.is_none() {
        return String::from("None");
    } else {
        return format!("Some({:})", option.unwrap().0);
    }
}