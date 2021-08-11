use crate::{
    data::{HasName, ValueWrapper},
    replace_references,
};
use askama::Template;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonCondition {
    data: ConditionData,
    name: String,
}

#[derive(Deserialize)]
pub struct ConditionData {
    description: ValueWrapper<String>,
}

#[derive(Deserialize, Debug, PartialEq, Template, Clone)]
#[serde(from = "JsonCondition")]
#[template(path = "condition.html", escape = "none")]
pub struct Condition {
    pub name: String,
    pub description: String,
}

impl HasName for Condition {
    fn name(&self) -> &str {
        &self.name
    }
}

impl From<JsonCondition> for Condition {
    fn from(jc: JsonCondition) -> Self {
        Condition {
            name: jc.name,
            description: replace_references(&jc.data.description.value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use std::io::BufReader;

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

        let archetype: Condition = serde_json::from_str::<Condition>(&json).unwrap();
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
        let f = std::fs::File::open("tests/data/conditions/blinded.json").expect("File missing");
        let reader = BufReader::new(f);
        let blinded: Condition = serde_json::from_reader(reader).expect("Deserialization failed");
        assert_eq!(blinded.name, String::from("Blinded"));
    }
}
