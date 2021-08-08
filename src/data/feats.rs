use serde::Deserialize;

use super::{
    action_type::ActionType,
    feat_type::FeatType,
    traits::{JsonTraits, Traits},
    HasName, StringWrapper, ValueWrapper,
};

#[derive(Deserialize, PartialEq, Debug, Clone)]
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

impl HasName for Feat {
    fn name(&self) -> &str {
        &self.name
    }
}

impl From<JsonFeat> for Feat {
    fn from(jf: JsonFeat) -> Self {
        Feat {
            name: jf.name,
            action_type: jf.data.action_type.value,
            actions: jf.data.actions.value.and_then(|s| s.parse().ok()),
            description: jf.data.description.value,
            feat_type: jf.data.feat_type.value,
            level: jf.data.level.value,
            prerequisites: jf.data.prerequisites.value.into_iter().map(|p| p.0).collect(),
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
    actions: ValueWrapper<Option<String>>,
    description: ValueWrapper<String>,
    feat_type: ValueWrapper<FeatType>,
    level: ValueWrapper<i32>,
    // The nested type here can’t be a ValueWrapper<String> because it isn’t always a wrapper.
    // In at least one example (demonblood-frenzy), this is a Vec<String>, so I needed to write a
    // custom deserializer.
    // TODO: Open a MR in the module to fix that
    prerequisites: ValueWrapper<Vec<StringWrapper>>,
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
