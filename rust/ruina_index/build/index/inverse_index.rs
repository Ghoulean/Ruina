use std::collections::HashMap;

use super::index::Index;

pub struct InverseIndex(pub HashMap<String, Vec<String>>);

impl InverseIndex {
    pub fn from_index(index: Index) -> InverseIndex {
        let Index(index_map) = index;
        let mut inverse_index: HashMap<String, Vec<String>> = HashMap::new();
        index_map.iter()
            .for_each(|(key, tags)| {
                tags.iter().for_each(|tag| {
                    let inverse_index_entry = inverse_index.entry(tag.clone()).or_insert(Vec::new());
                    if !inverse_index_entry.contains(key) {
                        inverse_index_entry.push(key.clone());
                    }
                })
            });
        InverseIndex(inverse_index)
    }

    pub fn to_serialized_phf_map(&self, var_name: &str) -> String {
        let mut builder = phf_codegen::Map::new();
        for (key, vec) in &self.0 {
            builder.entry(
                key,
                format!("[{}]", vec.join(",")).as_str()
            );
        }
        format!(
            "static {}: phf::Map<&'static str, phf::Map<&str, &[&str]>> = {};",
            var_name,
            builder.build()
        )
    }
}
