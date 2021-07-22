use super::{
    ability_scores::{AbilityBoost, JsonAbilityBoosts},
    size::Size,
    traits::Traits,
    ValueWrapper,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Ancestry {
    name: String,
    boosts: Vec<AbilityBoost>,
    flaws: Vec<AbilityBoost>,
    description: String,
    hp: i32,
    ancestry_features: Vec<AncestryItem>,
    languages: Vec<String>,
    additional_languages: Vec<String>,
    size: Size,
    speed: i32,
    traits: Traits,
}

impl From<JsonAncestry> for Ancestry {
    fn from(ja: JsonAncestry) -> Self {
        Ancestry {
            name: ja.name,
            boosts: ja.data.boosts.into(),
            flaws: ja.data.flaws.into(),
            description: ja.data.description.value,
            hp: ja.data.hp,
            ancestry_features: ja.data.ancestry_features.into_iter().map(|(_, v)| v).collect(),
            languages: ja.data.languages.value,
            additional_languages: ja.data.additional_languages.value,
            size: ja.data.size,
            speed: ja.data.speed,
            traits: ja.data.traits,
        }
    }
}

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
    ancestry_features: HashMap<String, AncestryItem>,
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
pub struct AncestryItem {
    name: String,
    pack: String,
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;
    use crate::data::ability_scores::AbilityScore;

    #[test]
    fn should_deserialize_ancestry() {
        let f = std::fs::File::open("tests/data/ancestries/anadi.json").expect("File missing");
        let reader = BufReader::new(f);
        let anadi: JsonAncestry = serde_json::from_reader(reader).expect("Deserialization failed");
        let anadi = Ancestry::from(anadi);
        assert_eq!(anadi.name, String::from("Anadi"));
        assert_eq!(anadi.size, Size::Medium);
        assert_eq!(anadi.flaws, vec![AbilityBoost(vec![AbilityScore::Constitution])]);
        assert_eq!(&anadi.ancestry_features[0].name, "Low-Light Vision");
    }
}
