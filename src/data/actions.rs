use super::equipment::StringOrNum;
use super::traits::Traits;
use crate::data::{action_type::ActionType, traits::JsonTraits, ValueWrapper};
use crate::text_cleanup;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct JsonAction {
    pub system: ActionData,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionData {
    action_type: ValueWrapper<ActionType>,
    description: ValueWrapper<String>,
    #[serde(rename = "actions")]
    number_of_actions: ValueWrapper<Option<StringOrNum>>,
    traits: JsonTraits,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(from = "JsonAction")]
pub struct Action {
    pub name: String,
    pub description: String,
    pub action_type: ActionType,
    pub number_of_actions: Option<i32>,
    pub traits: Traits,
}

impl From<JsonAction> for Action {
    fn from(ja: JsonAction) -> Self {
        Action {
            name: ja.name.clone(),
            description: text_cleanup(&ja.system.description.value),
            action_type: ja.system.action_type.value,
            number_of_actions: ja.system.number_of_actions.value.map(i32::from).filter(|&n| n != 0),
            traits: Traits::from(ja.system.traits),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{data::traits::Rarity, tests::read_test_file};

    #[test]
    fn should_deserialize_real_action() {
        let aid: Action = serde_json::from_str(&read_test_file("actions/aid.json")).expect("Deserialization failed");
        assert_eq!(aid.name, "Aid");
        assert_eq!(aid.action_type, ActionType::Reaction);
        assert_eq!(aid.number_of_actions, None);
        assert_eq!(
            aid.traits,
            Traits {
                misc: vec!["general".into()],
                rarity: Rarity::Common,
                size: None,
                alignment: None,
            }
        );
    }
}
