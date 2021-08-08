use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    Action,
    Reaction,
    Passive,
    Free,
}
