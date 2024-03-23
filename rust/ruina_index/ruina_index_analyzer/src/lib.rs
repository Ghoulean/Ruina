mod filters;
mod tokenizer;

use crate::filters::filter;
use crate::tokenizer::tokenize;
pub use crate::tokenizer::Token;

pub fn analyze(text: &str) -> Vec<Token> {
    filter(tokenize(text))
}
