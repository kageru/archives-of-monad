use super::traits::Traits;
use crate::data::{
    action_type::ActionType,
    traits::JsonTraits,
    {HasName, ValueWrapper},
};
use crate::replace_references;
use serde::Deserialize;

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
    number_of_actions: ValueWrapper<String>,
    traits: JsonTraits,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(from = "JsonAction")]
pub struct Action {
    pub name: String,
    pub description: String,
    pub action_type: ActionType,
    pub number_of_actions: Option<i32>,
    pub traits: Traits,
}

impl HasName for Action {
    fn name(&self) -> &str {
        &self.name
    }
}

impl From<JsonAction> for Action {
    fn from(ja: JsonAction) -> Self {
        let number_of_actions = ja.data.number_of_actions.value.parse::<i32>().ok();
        Action {
            name: ja.name,
            description: replace_references(&ja.data.description.value),
            action_type: ja.data.action_type.value,
            number_of_actions,
            traits: Traits::from(ja.data.traits),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::traits::Rarity;
    use serde_json::json;
    use std::io::BufReader;

    #[test]
    fn should_deserialize_action() {
        let json = json!(
        {
            "data": {
                "actionType": {
                    "value": "action"
                },
                "actions": {
                    "value": "1"
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
                    "value": ""
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
        let f = std::fs::File::open("tests/data/actions/aid.json").expect("File missing");
        let reader = BufReader::new(f);
        let aid: Action = serde_json::from_reader(reader).expect("Deserialization failed");
        assert_eq!(aid.name, String::from("Aid"));
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
