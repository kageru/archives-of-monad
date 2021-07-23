use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub enum ActionType {
    Action,
    Reaction,
    Passive,
    Free,
}

impl<'de> Deserialize<'de> for ActionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "action" => Ok(ActionType::Action),
            "reaction" => Ok(ActionType::Reaction),
            "passive" => Ok(ActionType::Passive),
            "free" => Ok(ActionType::Free),
            s => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(s),
                &"action|reaction|passive|free",
            )),
        }
    }
}
