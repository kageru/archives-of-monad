use crate::impl_deser;
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub enum ActionType {
    Action,
    Reaction,
    Passive,
    Free,
}

impl_deser! {
    ActionType :
    "action" => ActionType::Action,
    "reaction" => ActionType::Reaction,
    "passive" => ActionType::Passive,
    "free" => ActionType::Free,
    expects: "action|reaction|passive|free"
}
