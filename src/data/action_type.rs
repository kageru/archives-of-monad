use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    Action,
    Reaction,
    Passive,
    Free,
}
