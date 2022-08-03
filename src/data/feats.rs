use super::{
    action_type::ActionType,
    feat_type::FeatType,
    traits::{JsonTraits, Traits},
    HasLevel, ValueWrapper,
};
use crate::text_cleanup;
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
    pub source: String,
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
            action_type: jf.system.action_type.value,
            actions: jf.system.actions.value.filter(|&n| n != 0),
            description: text_cleanup(&jf.system.description.value),
            feat_type: jf.system.feat_type.value,
            level: jf.system.level.value,
            prerequisites: jf.system.prerequisites.value.into_iter().map(|p| p.value).collect(),
            traits: jf.system.traits.into(),
            source: jf.system.source.value,
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
struct JsonFeat {
    name: String,
    system: JsonFeatData,
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
    source: ValueWrapper<String>,
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
                rarity: Rarity::Uncommon,
                misc: vec![
                    String::from("conjuration"),
                    String::from("fighter"),
                    String::from("flourish"),
                    String::from("teleportation"),
                ],
                alignment: None,
                size: None,
            },
            sever_space.traits
        );
        assert_eq!(Some(2), sever_space.actions);
        assert_eq!(ActionType::Action, sever_space.action_type);
        assert_eq!(20, sever_space.level);
        assert_eq!("Pathfinder #168: King of the Mountain", sever_space.source);
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
