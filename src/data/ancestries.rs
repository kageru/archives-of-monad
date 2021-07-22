use super::{ability_scores::AbilityBoosts, size::Size, traits::Traits, ValueWrapper};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq)]
pub struct JsonAncestry {
    data: InnerJsonAncestry,
    name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct InnerJsonAncestry {
    #[serde(rename = "additionalLanguages")]
    additional_languages: AdditionalLanguages,
    boosts: AbilityBoosts,
    description: ValueWrapper<String>,
    flaws: AbilityBoosts,
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
    value: ValueWrapper<Vec<String>>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct JsonAncestryItem {
    name: String,
    pack: String,
}
