use super::traits::Traits;
use crate::data::{action_type::ActionType, traits::JsonTraits, ValueWrapper};
use crate::replace_references;
use meilisearch_sdk::document::Document;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct JsonAction {
    data: ActionData,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionData {
    action_type: ValueWrapper<ActionType>,
    description: ValueWrapper<String>,
    #[serde(rename = "actions")]
    number_of_actions: ValueWrapper<Option<i32>>,
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
    pub id: String,
}

impl Document for Action {
    type UIDType = String;
    fn get_uid(&self) -> &Self::UIDType {
        return &self.id;
    }
}

impl From<JsonAction> for Action {
    fn from(ja: JsonAction) -> Self {
        Action {
            name: ja.name.clone(),
            description: replace_references(&ja.data.description.value),
            action_type: ja.data.action_type.value,
            number_of_actions: ja.data.number_of_actions.value,
            traits: Traits::from(ja.data.traits),
            id: format!("action-{}", ja.name),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{data::traits::Rarity, tests::read_test_file};
    use serde_json::json;

    #[test]
    fn should_deserialize_action() {
        let json = json!(
        {
            "data": {
                "actionType": {
                    "value": "action"
                },
                "actions": {
                    "value": 1
                },
                "description": {
                    "value": "Testing"
                },
                "traits": {
                    "value": [
                        "lel",
                        "lel2"
                    ]
                }
            },
            "name": "Test"
        })
        .to_string();
        let action = serde_json::from_str::<Action>(&json).unwrap();
        assert_eq!(
            action,
            Action {
                name: "Test".into(),
                description: "Testing".into(),
                action_type: ActionType::Action,
                number_of_actions: Some(1),
                traits: Traits {
                    value: vec!["lel".into(), "lel2".into()],
                    rarity: None
                },
            }
        );
    }

    #[test]
    fn should_deserialize_action_without_number_of_actions() {
        let json = json!(
        {
            "data": {
                "actionType": {
                    "value": "reaction"
                },
                "actions": {
                    "value": null
                },
                "description": {
                    "value": "Testing"
                },
                "traits": {
                    "value": [
                        "lel",
                        "lel2"
                    ]
                }
            },
            "name": "Test"
        });

        let action: Action = serde_json::from_value(json).unwrap();
        assert_eq!(
            action,
            Action {
                name: "Test".into(),
                description: "Testing".into(),
                action_type: ActionType::Reaction,
                number_of_actions: None,
                traits: Traits {
                    value: vec!["lel".into(), "lel2".into()],
                    rarity: None
                },
            }
        );
    }

    #[test]
    fn should_deserialize_real_action() {
        let aid: Action = serde_json::from_str(&read_test_file("actions.db/aid.json")).expect("Deserialization failed");
        assert_eq!(aid.name, "Aid");
        assert_eq!(aid.action_type, ActionType::Reaction);
        assert_eq!(aid.number_of_actions, None);
        assert_eq!(
            aid.traits,
            Traits {
                value: vec!["general".into()],
                rarity: Some(Rarity::Common),
            }
        );
    }
}
