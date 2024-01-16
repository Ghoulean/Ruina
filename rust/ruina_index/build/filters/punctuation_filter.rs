use super::filter::Filter;

pub struct PunctuationFilter {}

impl PunctuationFilter {
    pub fn new() -> PunctuationFilter {
        PunctuationFilter {}
    }
}

impl Filter for PunctuationFilter {
    fn filter(&self, vec: Vec<String>) -> Vec<String> {
        vec.iter().map(|s| s.to_lowercase()).collect()
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