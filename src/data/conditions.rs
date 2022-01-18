use crate::{data::ValueWrapper, text_cleanup};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct JsonCondition {
    data: ConditionData,
    name: String,
}

#[derive(Deserialize)]
pub struct ConditionData {
    description: ValueWrapper<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(from = "JsonCondition")]
pub struct Condition {
    pub name: String,
    pub description: String,
}

impl From<JsonCondition> for Condition {
    fn from(jc: JsonCondition) -> Self {
        Condition {
            name: jc.name.clone(),
            description: text_cleanup(&jc.data.description.value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::read_test_file;
    use serde_json::json;

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
        });

        let archetype: Condition = serde_json::from_value(json).unwrap();
        assert_eq!(
            archetype,
            Condition {
                name: "Tester".into(),
                description: "Testing".into(),
            }
        );
    }

    #[test]
    fn should_deserialize_real_condition() {
        let blinded: Condition = serde_json::from_str(&read_test_file("conditionitems.db/blinded.json")).expect("Deserialization failed");
        assert_eq!(blinded.name, String::from("Blinded"));
    }
}
