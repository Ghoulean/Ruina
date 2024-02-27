use std::{env, fs::File, io::Write, path::PathBuf};

use index::index::Index;
use ruina_common::localizations::common::Locale;
use rust_stemmers::{Algorithm, Stemmer};
use strum::IntoEnumIterator;

use ruina_reparser::{ABNO_PAGES, BATTLE_SYMBOLS, COMBAT_PAGES, KEY_PAGES, PASSIVES};
use taggers::tagger::Tag;

use crate::taggers::tagger::Tagger;
use crate::{
    analyzer::{
        analyze::Analyzer,
        filters::{
            filter::Filter, punctuation_filter::PunctuationFilter, stemming_filter::StemmingFilter,
            stopword_filter::StopwordFilter,
        },
        tokenizer::tokenizer::Tokenizer,
    },
    autocomplete::autocomplete::generate_serialized_autocomplete_objs,
    index::inverse_index::InverseIndex,
};

mod analyzer;
mod autocomplete;
mod index;
mod taggers;

fn main() {
    let out_file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(PathBuf::from("out.rs"));
    if false && out_file_path.exists() {
        dbg!(
            "[index] not rebuilding because artifacts already exists",
            out_file_path.to_str().unwrap()
        );
        return;
    }
    let mut out_file = File::create(out_file_path).unwrap();

    let en_stemmer = Stemmer::create(Algorithm::English);
    let filters: Vec<Box<dyn Filter>> = vec![
        Box::new(PunctuationFilter::new()),
        Box::new(StemmingFilter::new(en_stemmer)),
        Box::new(StopwordFilter::new()),
    ];

    let tokenizer = Tokenizer::new();

    let analyzer = Analyzer::new(tokenizer, filters);

    let index = build_index_from(Vec::from(ABNO_PAGES), &analyzer)
        .merge(build_index_from(Vec::from(BATTLE_SYMBOLS), &analyzer))
        .merge(build_index_from(Vec::from(COMBAT_PAGES), &analyzer))
        .merge(build_index_from(Vec::from(KEY_PAGES), &analyzer))
        .merge(build_index_from(Vec::from(PASSIVES), &analyzer));

    let idk = index.clone();

    dbg!("{:?}", idk);

    let inverse_index = InverseIndex::from_index(index);

    let autocomplete_objs = Locale::iter()
        .map(|x| generate_serialized_autocomplete_objs(&x))
        .collect::<Vec<_>>()
        .join("\n");

    let output = [
        inverse_index.to_serialized_phf_map("INVERSE_CARD_INDEX"),
        autocomplete_objs
    ]
    .join("\n");

    out_file.write_all(output.as_bytes()).unwrap();
    dbg!("[reparser] wrote artifacts");
}

fn build_index_from(taggers: Vec<impl Tagger>, analyzer: &Analyzer) -> Index {
    Index(
        taggers
            .iter()
            .map(|x| {
                let typed_id = x.get_typed_id();
                let tags = 
                x.generate_tags()
                    .iter()
                    .map(|tag| tag.0.clone())
                    .flat_map(|txt| analyzer.analyze(&txt))
                    .map(|token| token.0)
                    .map(Tag)
                    .collect();
                dbg!("typed_id=", typed_id);
                (
                    x.get_typed_id(),
                    tags
                )
            })
            .collect(),
    )
}
