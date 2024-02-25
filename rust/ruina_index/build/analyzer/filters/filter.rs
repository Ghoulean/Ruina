use crate::analyzer::tokenizer::tokenizer::Token;

pub trait Filter {
    fn filter(&self, vecs: Vec<Token>) -> Vec<Token>;
}
