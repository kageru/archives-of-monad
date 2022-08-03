use crate::{data::ValueWrapper, text_cleanup};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct JsonCondition {
    system: ConditionData,
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
            description: text_cleanup(&jc.system.description.value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::read_test_file;

    #[test]
    fn should_deserialize_real_condition() {
        let blinded: Condition = serde_json::from_str(&read_test_file("conditionitems.db/blinded.json")).expect("Deserialization failed");
        assert_eq!(blinded.name, String::from("Blinded"));
    }
}
