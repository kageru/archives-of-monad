use crate::INDEX_REGEX;
use crate::{data::ValueWrapper, replace_references};
use meilisearch_sdk::document::Document;
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
    pub id: String,
}

impl Document for Condition {
    type UIDType = String;
    fn get_uid(&self) -> &Self::UIDType {
        &self.id
    }
}

impl From<JsonCondition> for Condition {
    fn from(jc: JsonCondition) -> Self {
        Condition {
            name: jc.name.clone(),
            description: replace_references(&jc.data.description.value),
            id: format!("condition-{}", INDEX_REGEX.replace_all(&jc.name, "")),
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
                id: "condition-Tester".into(),
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
