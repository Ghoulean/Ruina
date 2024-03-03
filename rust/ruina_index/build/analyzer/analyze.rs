use crate::analyzer::filters::filter::Filter;
use crate::analyzer::tokenizer::tokenizer::Tokenizer;

use super::tokenizer::tokenizer::Token;

pub struct Analyzer {
    tokenizer: Tokenizer,
    filters: Vec<Box<dyn Filter>>,
}

impl Analyzer {
    pub fn new(tokenizer: Tokenizer, filters: Vec<Box<dyn Filter>> ) -> Analyzer {
        Analyzer {
            tokenizer: tokenizer,
            filters: filters
        }
    }

    pub fn analyze(&self, text: &str) -> Vec<Token> {
        let mut tokens = self.tokenizer.tokenize(text);
        self.filters.iter().for_each(|f| {
            tokens = f.filter(tokens.clone());
        });
        return tokens
    }
}
