use std::collections::HashMap;

pub struct Index(pub HashMap<String, Vec<String>>);

impl Index {
    pub fn merge(&self, other: Index) -> Index {
        Index(self.0.clone().into_iter().chain(other.0).collect())
    }
}