// TODO: figure out typings
// and whether or not trait actually works here
pub trait Tagger {
    fn generate_tag_key(&self) -> String;
    fn generate_tags(&self) -> Vec<String>;
}
