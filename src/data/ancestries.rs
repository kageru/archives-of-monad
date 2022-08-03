use super::{
    ability_scores::{AbilityBoost, JsonAbilityBoosts},
    size::Size,
    traits::Traits,
    ValueWrapper,
};
use crate::{data::traits::JsonTraits, text_cleanup};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(from = "JsonAncestry")]
pub struct Ancestry {
    pub name: String,
    pub boosts: Vec<AbilityBoost>,
    pub flaws: Vec<AbilityBoost>,
    pub description: String,
    pub hp: i32,
    pub ancestry_features: Vec<AncestryItem>,
    pub languages: Vec<String>,
    pub additional_languages: Vec<String>,
    pub num_of_additional_languages: i32,
    pub size: Size,
    pub speed: i32,
    pub traits: Traits,
    pub source: String,
}

impl From<JsonAncestry> for Ancestry {
    fn from(ja: JsonAncestry) -> Self {
        Ancestry {
            name: ja.name.clone(),
            boosts: ja.system.boosts.into(),
            flaws: ja.system.flaws.into(),
            description: text_cleanup(&ja.system.description.value),
            hp: ja.system.hp,
            ancestry_features: ja
                .system
                .ancestry_features
                .into_values()
                .sorted_by_key(|af| af.name.clone()) // For consistent rendering order
                .collect(),
            languages: ja.system.languages.value,
            additional_languages: ja.system.additional_languages.value,
            num_of_additional_languages: ja.system.additional_languages.count,
            size: ja.system.size,
            speed: ja.system.speed,
            traits: ja.system.traits.into(),
            source: ja.system.source.value,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct JsonAncestry {
    system: InnerJsonAncestry,
    name: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
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
    source: ValueWrapper<String>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AdditionalLanguages {
    count: i32,
    value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AncestryItem {
    pub(crate) name: String,
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
                misc: vec!["anadi".into(), "humanoid".into()],
                rarity: Rarity::Rare,
                size: None,
                alignment: None,
            }
        );
        assert_eq!(anadi.source, "Pathfinder Lost Omens: The Mwangi Expanse");
    }
}
