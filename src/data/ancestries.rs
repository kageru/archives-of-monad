use super::{
    ability_scores::{AbilityBoost, JsonAbilityBoosts},
    size::Size,
    traits::Traits,
    ValueWrapper,
};
use crate::data::traits::JsonTraits;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(from = "JsonAncestry")]
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
            name: ja.name.clone(),
            boosts: ja.data.boosts.into(),
            flaws: ja.data.flaws.into(),
            description: ja.data.description.value,
            hp: ja.data.hp,
            ancestry_features: ja.data.ancestry_features.into_values().collect(),
            languages: ja.data.languages.value,
            additional_languages: ja.data.additional_languages.value,
            size: ja.data.size,
            speed: ja.data.speed,
            traits: ja.data.traits.into(),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct JsonAncestry {
    data: InnerJsonAncestry,
    name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InnerJsonAncestry {
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
    traits: JsonTraits,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AdditionalLanguages {
    count: i32,
    value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AncestryItem {
    name: String,
    pack: String,
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    use crate::data::ability_scores::AbilityScore;
    use crate::data::traits::Rarity;
    use crate::tests::read_test_file;

    #[test]
    fn should_deserialize_ancestry() {
        let anadi: Ancestry = serde_json::from_str(&read_test_file("ancestries.db/anadi.json")).expect("Deserialization failed");
        assert_eq!(anadi.name, String::from("Anadi"));
        assert_eq!(anadi.size, Size::Medium);
        assert_eq!(anadi.flaws, vec![AbilityBoost(vec![AbilityScore::Constitution])]);
        assert_eq!(
            &anadi.ancestry_features.iter().map(|f| &f.name).sorted().collect::<Vec<_>>(),
            &["Change Shape (Anadi)", "Fangs"]
        );
        assert_eq!(
            anadi.traits,
            Traits {
                value: vec!["anadi".into(), "humanoid".into()],
                rarity: Some(Rarity::Rare),
            }
        );
    }
}
