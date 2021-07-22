use super::{ability_scores::JsonAbilityBoosts, size::Size, traits::Traits, ValueWrapper};
use serde::Deserialize;
use std::collections::HashMap;

//#[derive(Debug, PartialEq)]
//pub struct Ancestry {
    //name: String,
    //boosts: JsonAbilityBoosts
//}

#[derive(Deserialize, Debug, PartialEq)]
pub struct JsonAncestry {
    data: InnerJsonAncestry,
    name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct InnerJsonAncestry {
    #[serde(rename = "additionalLanguages")]
    additional_languages: AdditionalLanguages,
    boosts: JsonAbilityBoosts,
    description: ValueWrapper<String>,
    flaws: JsonAbilityBoosts,
    hp: i32,
    #[serde(rename = "items")]
    ancestry_features: HashMap<String, JsonAncestryItem>,
    languages: ValueWrapper<Vec<String>>,
    size: Size,
    speed: i32,
    traits: Traits,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct AdditionalLanguages {
    count: i32,
    value: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct JsonAncestryItem {
    name: String,
    pack: String,
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn should_deserialize_ancestry() {
        let f = std::fs::File::open("tests/data/ancestries/anadi.json").expect("File missing");
        let reader = BufReader::new(f);
        let anadi: JsonAncestry = serde_json::from_reader(reader).expect("Deserialization failed");
        assert_eq!(anadi.name, String::from("Anadi"));
    }
}
