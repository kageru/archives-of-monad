use askama::Template;
use serde::Deserialize;

use super::{
    action_type::ActionType,
    feat_type::FeatType,
    traits::{JsonTraits, Traits},
    ValueWrapper,
};
use crate::data::traits::{Trait, TraitDescriptions};
use convert_case::{Case, Casing};

#[derive(Template, PartialEq, Debug)]
#[template(path = "feat.html", escape = "none")]
pub struct FeatTemplate {
    pub name: String,
    pub action_type: ActionType,
    pub actions: Option<i32>,
    pub description: String,
    pub feat_type: FeatType,
    pub level: i32,
    pub prerequisites: Vec<String>,
    pub traits: Vec<Trait>,
}

impl FeatTemplate {
    pub fn new(feat: Feat, trait_descriptions: &TraitDescriptions) -> Self {
        let test = feat
            .traits
            .value
            .iter()
            .map(|name| name.to_case(Case::Pascal))
            .map(|name| Trait {
                description: trait_descriptions.0[&name].clone(),
                name,
            })
            .collect();

        FeatTemplate {
            name: feat.name,
            action_type: feat.action_type,
            actions: feat.actions,
            description: feat.description,
            feat_type: feat.feat_type,
            level: feat.level,
            prerequisites: feat.prerequisites,
            traits: test,
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(from = "JsonFeat")]
pub struct Feat {
    pub name: String,
    pub action_type: ActionType,
    pub actions: Option<i32>,
    pub description: String,
    pub feat_type: FeatType,
    pub level: i32,
    pub prerequisites: Vec<String>,
    pub traits: Traits,
}

impl From<JsonFeat> for Feat {
    fn from(jf: JsonFeat) -> Self {
        Feat {
            name: jf.name,
            action_type: jf.data.action_type.value,
            actions: jf.data.actions.value.parse().ok(),
            description: jf.data.description.value,
            feat_type: jf.data.feat_type.value,
            level: jf.data.level.value,
            prerequisites: jf.data.prerequisites.value.into_iter().map(|p| p.value).collect(),
            traits: jf.data.traits.into(),
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
struct JsonFeat {
    name: String,
    data: JsonFeatData,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct JsonFeatData {
    action_type: ValueWrapper<ActionType>,
    #[serde(default)]
    actions: ValueWrapper<String>,
    description: ValueWrapper<String>,
    feat_type: ValueWrapper<FeatType>,
    level: ValueWrapper<i32>,
    prerequisites: ValueWrapper<Vec<ValueWrapper<String>>>,
    traits: JsonTraits,
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::data::traits::Rarity;

    use super::*;

    #[test]
    fn test_sever_space_deserialization() {
        let f = std::fs::File::open("tests/data/feats/sever-space.json").expect("File missing");
        let reader = BufReader::new(f);
        let sever_space: Feat = serde_json::from_reader(reader).expect("Deserialization failed");
        assert_eq!("Sever Space", sever_space.name.as_str());
        assert_eq!(
            Traits {
                rarity: Some(Rarity::Uncommon),
                value: vec![
                    String::from("conjuration"),
                    String::from("fighter"),
                    String::from("flourish"),
                    String::from("teleportation"),
                ]
            },
            sever_space.traits
        );
        assert_eq!(Some(2), sever_space.actions);
        assert_eq!(ActionType::Action, sever_space.action_type);
        assert_eq!(20, sever_space.level);
    }
    #[test]
    fn test_champion_dedication_deserialization() {
        let f = std::fs::File::open("tests/data/feats/champion-dedication.json").expect("File missing");
        let reader = BufReader::new(f);
        let champion_dedication: Feat = serde_json::from_reader(reader).expect("Deserialization failed");
        assert_eq!("Champion Dedication", champion_dedication.name.as_str());
        assert_eq!(
            vec![String::from("Strength 14"), String::from("Charisma 14")],
            champion_dedication.prerequisites,
        );
    }
}
