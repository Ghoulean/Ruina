use std::collections::HashSet;

use crate::analyzer::tokenizer::tokenizer::Token;

use super::filter::Filter;

pub struct StopwordFilter {
   stopwords: HashSet<&'static str>
}

impl StopwordFilter {
    pub fn new() -> StopwordFilter {
        StopwordFilter {
            stopwords: HashSet::from(["a", "the", "of"])
        }
    }
}

impl Filter for StopwordFilter {
    /// Prerequisite: all strings in `vec` are lowercase and no punctuation
    /// (TODO: breaks encapsulation -- rethink)
    /// Don't understand why unwrapping and rewrapping is necessary -- investigate
    fn filter(&self, vec: Vec<Token>) -> Vec<Token> {
        vec.iter()
            .map(|token| token.0.clone())
            .filter(|s| !self.stopwords.contains(s.as_str()))
            .map(Token)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let under_test = StopwordFilter::new();

        let input = vec!["the", "weight", "of", "sin"];
        let expected = vec!["weight", "sin"];

        assert_eq!(expected, under_test.filter(input));
    }
}
