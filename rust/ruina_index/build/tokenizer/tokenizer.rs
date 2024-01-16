// TODO: move to another crate
pub struct Tokenizer {}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer{}
    }

    pub fn tokenize(&self, txt: &str) -> Vec<String> {
        txt.split(" ").map(String::from).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let under_test = Tokenizer::new();

        let input = "The Weight of Sin";
        let expected = vec![
            "The".to_string(),
            "Weight".to_string(),
            "of".to_string(),
            "Sin".to_string(),
        ];
        assert_eq!(expected, under_test.tokenize(input));
    }
}