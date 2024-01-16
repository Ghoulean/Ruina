// TODO: move to another crate
// TODO: newtype
pub trait Filter {
    fn filter(&self, vecs: Vec<String>) -> Vec<String>;
}
