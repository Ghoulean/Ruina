use rust_stemmers::Stemmer;

use crate::analyzer::tokenizer::tokenizer::Token;

use super::filter::Filter;

pub struct StemmingFilter {
    en_stemmer: Stemmer,
}

impl StemmingFilter {
    pub fn new(en_stemmer: Stemmer) -> StemmingFilter {
        StemmingFilter {
            en_stemmer: en_stemmer,
        }
    }
}

impl Filter for StemmingFilter {
    /// Prerequisite: all tokens in `vec` are lowercase and have no punctuation
    /// (TODO: breaks encapsulation -- rethink)
    fn filter(&self, vec: Vec<Token>) -> Vec<Token> {
        vec.iter()
            .map(|s| self.en_stemmer.stem(&s.0))
            .map(String::from)
            .map(Token)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let under_test = StemmingFilter::new(Stemmer::create(Algorithm::English));

        let input = vec!["senses"];
        let expected = vec!["sense"];

        assert_eq!(expected, under_test.filter(input));
    }
}
