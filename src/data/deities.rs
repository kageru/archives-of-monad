use serde::{Deserialize, Serialize};

use crate::text_cleanup;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(from = "JsonDeity")]
pub struct Deity {
    pub content: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
struct JsonDeity {
    content: String,
    name: String,
}

impl From<JsonDeity> for Deity {
    fn from(jd: JsonDeity) -> Self {
        Deity {
            content: text_cleanup(&jd.content, false),
            name: jd.name,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tests::read_test_file;

    use super::*;
    use serde_json::json;

    #[test]
    fn should_deserialize_deity() {
        let json = json!(
        {
            "content": "Testing",
            "name": "Tester"
        });

        let deity: Deity = serde_json::from_value(json).unwrap();
        assert_eq!(
            deity,
            Deity {
                name: "Tester".into(),
                content: "Testing".into(),
            }
        );
    }

    #[test]
    fn should_deserialize_real_deity() {
        let asmodeus: Deity = serde_json::from_str(&read_test_file("deities.db/asmodeus.json")).expect("Deserialization failed");
        assert_eq!(asmodeus.name, String::from("Asmodeus"));
    }
}
