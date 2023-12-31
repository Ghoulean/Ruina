use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::reserializer::commons::paths::{game_obj_path, read_xml_files_in_dir};
use crate::reserializer::commons::serde::{
    get_rarity_from_str, serialize_option, serialize_option_debug,
};
use crate::reserializer::commons::xml::{get_nodes, get_unique_node, get_unique_node_text};
use roxmltree::{Document, Node};

fn passive_path() -> &'static PathBuf {
    static PASSIVE_PATH: OnceLock<PathBuf> = OnceLock::new();
    PASSIVE_PATH.get_or_init(|| {
        fs::canonicalize(game_obj_path().as_path().join(PathBuf::from("PassiveList"))).unwrap()
    })
}

pub fn reserialize_passives() -> String {
    let passives: Vec<String> = read_xml_files_in_dir(passive_path())
        .into_iter()
        .map(|path_and_document_string| path_and_document_string.1)
        .flat_map(|document_string| process_passive_file(document_string.as_str()))
        .collect();
    format!(
        "pub const PASSIVES: [Passive; {}] = [{}];",
        passives.len(),
        passives.join(",")
    )
}

fn process_passive_file(document_string: &str) -> Vec<String> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "PassiveXmlRoot").unwrap();
    let passive_node_list = get_nodes(xml_root_node, "Passive");

    passive_node_list
        .into_iter()
        .map(|x| parse_passive(x))
        .collect()
}

fn parse_passive(passive_node: Node) -> String {
    let id = passive_node.attribute("ID").unwrap();
    let cost = serialize_option(get_unique_node_text(passive_node, "Cost"));
    let rarity = serialize_option_debug(
        "Rarity",
        get_unique_node_text(passive_node, "Rarity").map(get_rarity_from_str),
    );
    let hidden =
        serialize_option(get_unique_node_text(passive_node, "IsHide").map(|x| x == "true"));
    let transferable =
        serialize_option(get_unique_node_text(passive_node, "CanGivePassive").map(|x| x == "true"));
    let inner_type = serialize_option(get_unique_node_text(passive_node, "InnerType"));

    format!(
        "Passive {{
        id: \"{id}\",
        cost: {cost},
        rarity: {rarity},
        hidden: {hidden},
        transferable: {transferable},
        inner_type: {inner_type}
    }}"
    )
}
