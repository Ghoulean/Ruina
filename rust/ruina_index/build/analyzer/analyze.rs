use crate::filters::filter::Filter;
use crate::tokenizer::tokenizer::Tokenizer;

// TODO: definitely move this to its own crate
// TODO: newtype
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

    pub fn analyze(&self, text: &str) -> Vec<String> {
        let mut tokens = self.tokenizer.tokenize(text);
        self.filters.iter().for_each(|f| {
            tokens = f.filter(tokens.clone());
        });
        return tokens
    }
}
