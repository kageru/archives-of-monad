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

        let archetype: Condition = Condition::from(serde_json::from_str::<JsonCondition>(&json).unwrap());
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
        let blinded: JsonCondition = serde_json::from_reader(reader).expect("Deserialization failed");
        let blinded = Condition::from(blinded);
        assert_eq!(blinded.name, String::from("Blinded"));
    }
}
