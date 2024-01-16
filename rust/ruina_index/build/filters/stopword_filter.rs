use std::collections::HashSet;

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
    fn filter(&self, vec: Vec<String>) -> Vec<String> {
        vec.iter()
            .filter(|s| !self.stopwords.contains(s.as_str()))
            .map(String::from)
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
