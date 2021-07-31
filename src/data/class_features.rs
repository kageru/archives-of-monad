use crate::data::action_type::ActionType;
use crate::data::feat_type::FeatType;
use crate::data::traits::{JsonTraits, Traits};
use crate::data::ValueWrapper;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonClassFeature {
    data: ClassFeatureData,
    name: String,
}

#[derive(Deserialize)]
pub struct ClassFeatureData {
    #[serde(rename = "actionType")]
    action_type: ValueWrapper<ActionType>,
    #[serde(rename = "actions")]
    number_of_actions: ValueWrapper<String>,
    description: ValueWrapper<String>,
    level: ValueWrapper<i32>,
    #[serde(rename = "featType")]
    feat_type: ValueWrapper<FeatType>,
    traits: JsonTraits,
}

#[derive(Debug, PartialEq)]
pub struct ClassFeature {
    name: String,
    description: String,
    feat_type: FeatType,
    action_type: ActionType,
    number_of_actions: Option<i32>,
    level: i32,
    traits: Traits,
}

impl From<JsonClassFeature> for ClassFeature {
    fn from(jcf: JsonClassFeature) -> Self {
        let number_of_actions = jcf.data.number_of_actions.value.parse::<i32>().ok();
        ClassFeature {
            name: jcf.name,
            description: jcf.data.description.value,
            feat_type: jcf.data.feat_type.value,
            action_type: jcf.data.action_type.value,
            level: jcf.data.level.value,
            number_of_actions,
            traits: Traits::from(jcf.data.traits),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::traits::Rarity;
    use std::io::BufReader;

    #[test]
    fn should_deserialize_real_class_feature() {
        let f = std::fs::File::open("tests/data/features/rage.json").expect("File missing");
        let reader = BufReader::new(f);
        let rage: JsonClassFeature = serde_json::from_reader(reader).expect("Deserialization failed");
        let rage = ClassFeature::from(rage);
        assert_eq!(rage.name, String::from("Rage"));
        assert_eq!(rage.feat_type, FeatType::ClassFeature);
        assert_eq!(rage.action_type, ActionType::Action);
        assert_eq!(rage.level, 1);
        assert_eq!(rage.number_of_actions, Some(1));
        assert_eq!(
            rage.traits,
            Traits {
                value: vec!["barbarian".into(), "concentrate".into(), "emotion".into(), "mental".into()],
                rarity: Some(Rarity::Common),
            }
        );
    }
}