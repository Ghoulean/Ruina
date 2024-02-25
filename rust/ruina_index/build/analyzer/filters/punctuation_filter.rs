use crate::analyzer::tokenizer::tokenizer::Token;

use super::filter::Filter;

pub struct PunctuationFilter {}

impl PunctuationFilter {
    pub fn new() -> PunctuationFilter {
        PunctuationFilter {}
    }
}

impl Filter for PunctuationFilter {
    fn filter(&self, vec: Vec<Token>) -> Vec<Token> {
        vec.iter().map(|s| s.0.to_lowercase()).map(Token).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let under_test = PunctuationFilter::new();

        let input = vec!["hello?"];
        let expected = vec!["hello"];

        assert_eq!(expected, under_test.filter(input));
    }
}