use super::{
    action_type::ActionType,
    feat_type::FeatType,
    traits::{JsonTraits, Traits},
    HasLevel, ValueWrapper,
};
use crate::replace_references;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
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

impl HasLevel for Feat {
    fn level(&self) -> i32 {
        self.level
    }
}

impl From<JsonFeat> for Feat {
    fn from(jf: JsonFeat) -> Self {
        Feat {
            name: jf.name.clone(),
            action_type: jf.data.action_type.value,
            actions: jf.data.actions.value,
            description: replace_references(&jf.data.description.value),
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
    actions: ValueWrapper<Option<i32>>,
    description: ValueWrapper<String>,
    feat_type: ValueWrapper<FeatType>,
    level: ValueWrapper<i32>,
    prerequisites: ValueWrapper<Vec<ValueWrapper<String>>>,
    traits: JsonTraits,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{data::traits::Rarity, tests::read_test_file};

    #[test]
    fn test_sever_space_deserialization() {
        let sever_space: Feat = serde_json::from_str(&read_test_file("feats.db/sever-space.json")).expect("Deserialization failed");
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
        let champion_dedication: Feat =
            serde_json::from_str(&read_test_file("feats.db/champion-dedication.json")).expect("Deserialization failed");
        assert_eq!("Champion Dedication", champion_dedication.name.as_str());
        assert_eq!(
            vec![String::from("Strength 14"), String::from("Charisma 14")],
            champion_dedication.prerequisites,
        );
    }
}
