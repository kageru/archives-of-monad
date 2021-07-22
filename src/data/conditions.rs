use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct JsonCondition {
    data: ConditionData,
    name: String,
}

#[derive(Deserialize)]
pub struct ConditionData {
    description: ConditionDescription,
}

#[derive(Deserialize)]
pub struct ConditionDescription {
    value: String,
}

#[derive(Debug)]
pub struct Condition {
    name: String,
    description: String,
}

impl From<JsonCondition> for Condition {
    fn from(jc: JsonCondition) -> Self {
        Condition {
            name: jc.name,
            description: jc.data.description.value,
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}
