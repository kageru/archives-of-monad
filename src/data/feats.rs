use serde::Deserialize;

use super::{
    action_type::ActionType,
    feat_type::FeatType,
    traits::{JsonTraits, Traits},
    ValueWrapper,
};

#[derive(Deserialize, PartialEq, Debug)]
#[serde(from = "JsonFeat")]
struct Feat {
    name: String,
    action_type: ActionType,
    actions: Option<i32>,
    description: String,
    feat_type: FeatType,
    level: i32,
    prerequisites: Vec<String>,
    traits: Traits,
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