use rust_stemmers::Stemmer;

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
    /// Prerequisite: all strings in `vec` are lowercase and have no punctuation
    fn filter(&self, vec: Vec<String>) -> Vec<String> {
        vec.iter()
            .map(|s| self.en_stemmer.stem(s))
            .map(String::from)
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
