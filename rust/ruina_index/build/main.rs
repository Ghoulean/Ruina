use std::{path::PathBuf, env, fs::File, io::Write};

use analyzer::analyze::Analyzer;
use filters::{
    filter::Filter, punctuation_filter::PunctuationFilter, stemming_filter::StemmingFilter,
    stopword_filter::StopwordFilter,
};
use index::{index::Index, inverse_index::InverseIndex};
use ruina_reparser::{ABNO_PAGES, BATTLE_SYMBOLS, COMBAT_PAGES, KEY_PAGES, PASSIVES};
use rust_stemmers::{Algorithm, Stemmer};
use taggers::tagger::Tagger;
use tokenizer::tokenizer::Tokenizer;

mod analyzer;
mod filters;
mod index;
mod taggers;
mod tokenizer;

fn main() {
    let out_file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(PathBuf::from(env::var("OUT_FILE").unwrap()));
    if out_file_path.exists() {
        dbg!(
            "[index] artifacts already exist at {}; not rebuilding",
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

    let inverse_index = InverseIndex::from_index(index);

    let output = [
        inverse_index.to_serialized_phf_map("INVERSE_CARD_INDEX")
    ]
    .join("\n");

    out_file.write_all(output.as_bytes()).unwrap();
    dbg!("[reparser] wrote artifacts");
}

fn build_index_from(tagger: Vec<impl Tagger>, analyzer: &Analyzer) -> Index {
    Index(
        tagger
            .iter()
            .map(|x| {
                (
                    x.generate_tag_key(),
                    x.generate_tags()
                        .iter()
                        .flat_map(|txt| analyzer.analyze(txt))
                        .collect(),
                )
            })
            .collect(),
    )
}
