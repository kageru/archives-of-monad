use crate::data::ValueWrapper;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct JsonCondition {
    data: ConditionData,
    name: String,
}

#[derive(Deserialize)]
pub struct ConditionData {
    description: ValueWrapper<String>,
}

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_deserialize_condition() {
        let json = json!(
        {
            "data": {
                "description": {
                    "value": "Testing"
                }
            },
            "name": "Tester"
        })
        .to_string();

        let archetype: Condition = Condition::from(serde_json::from_str::<JsonCondition>(&json).unwrap());
        assert_eq!(
            archetype,
            Condition {
                name: "Tester".into(),
                description: "Testing".into(),
            }
        );
    }
}
