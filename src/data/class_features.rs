use crate::data::action_type::ActionType;
use crate::data::feat_type::FeatType;
use crate::data::traits::{JsonTraits, Traits};
use crate::data::ValueWrapper;
use crate::text_cleanup;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref LEVEL_ANNOTATION: Regex = Regex::new(r" \(Level \d+\)").unwrap();
}

#[derive(Deserialize)]
pub struct JsonClassFeature {
    system: ClassFeatureData,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassFeatureData {
    action_type: ValueWrapper<ActionType>,
    #[serde(rename = "actions")]
    number_of_actions: ValueWrapper<Option<i32>>,
    description: ValueWrapper<String>,
    level: ValueWrapper<i32>,
    feat_type: ValueWrapper<FeatType>,
    traits: JsonTraits,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(from = "JsonClassFeature")]
pub struct ClassFeature {
    pub name: String,
    pub description: String,
    pub feat_type: FeatType,
    pub action_type: ActionType,
    pub number_of_actions: Option<i32>,
    pub level: i32,
    pub traits: Traits,
}

impl From<JsonClassFeature> for ClassFeature {
    fn from(jcf: JsonClassFeature) -> Self {
        ClassFeature {
            name: LEVEL_ANNOTATION.replace_all(&jcf.name, "").to_string(),
            description: text_cleanup(&jcf.system.description.value),
            feat_type: jcf.system.feat_type.value,
            action_type: jcf.system.action_type.value,
            level: jcf.system.level.value,
            number_of_actions: jcf.system.number_of_actions.value,
            traits: Traits::from(jcf.system.traits),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{data::traits::Rarity, tests::read_test_file};

    #[test]
    fn should_deserialize_real_class_feature() {
        let rage: ClassFeature = serde_json::from_str(&read_test_file("classfeatures.db/rage.json")).expect("Deserialization failed");
        assert_eq!(rage.name, String::from("Rage"));
        assert_eq!(rage.feat_type, FeatType::ClassFeature);
        assert_eq!(rage.action_type, ActionType::Action);
        assert_eq!(rage.level, 1);
        assert_eq!(rage.number_of_actions, Some(1));
        assert_eq!(
            rage.traits,
            Traits {
                misc: vec!["barbarian".into(), "concentrate".into(), "emotion".into(), "mental".into()],
                rarity: Rarity::Common,
                size: None,
                alignment: None,
            }
        );
    }
}
