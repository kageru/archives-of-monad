use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct JsonAction {
    data: ActionData,
    name: String,
}

#[derive(Deserialize)]
pub struct ActionData {
    #[serde(rename = "actionType")]
    action_type: ActionType,
    description: ActionDescription,
    #[serde(rename = "actions")]
    number_of_actions: NumberOfActions,
    traits: Traits,
}

#[derive(Deserialize)]
pub struct ActionType {
    value: String,
}

#[derive(Deserialize)]
pub struct NumberOfActions {
    value: String,
}

#[derive(Deserialize)]
pub struct ActionDescription {
    value: String,
}

#[derive(Deserialize)]
pub struct Traits {
    value: Vec<String>,
}

#[derive(Debug)]
pub struct Action {
    name: String,
    description: String,
    action_type: String,
    number_of_actions: Option<i32>,
    traits: Vec<String>,
}

impl From<JsonAction> for Action {
    fn from(ja: JsonAction) -> Self {
        let number_of_actions = if ja.data.number_of_actions.value == "" {
            None
        } else {
            Some(
                ja.data
                    .number_of_actions
                    .value
                    .parse::<i32>()
                    .expect("This field has to be a number"),
            )
        };

        Action {
            name: ja.name,
            description: ja.data.description.value,
            action_type: ja.data.action_type.value,
            number_of_actions,
            traits: ja.data.traits.value,
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(actions) = self.number_of_actions {
            write!(f, "{}: {}, {}, {}", self.name, self.action_type, actions, self.description)
        } else {
            write!(f, "{}: {}, {}", self.name, self.action_type, self.description)
        }
    }
}
