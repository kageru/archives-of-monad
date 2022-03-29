use super::{creature::Alignment, ValueWrapper};
use crate::text_cleanup;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(from = "JsonDeity")]
pub struct Deity {
    pub content: String,
    pub name: String,
    pub alignment: Alignment,
    pub follower_alignments: Vec<Alignment>,
}

#[derive(Deserialize, Debug)]
struct JsonDeity {
    name: String,
    data: JsonDeityData,
}

#[derive(Deserialize, Debug)]
struct JsonDeityData {
    description: ValueWrapper<String>,
    alignment: JsonDeityAlignment,
}

#[derive(Deserialize, Debug)]
struct JsonDeityAlignment {
    own: Alignment,
    follower: Vec<Alignment>,
}

impl From<JsonDeity> for Deity {
    fn from(jd: JsonDeity) -> Self {
        Deity {
            content: text_cleanup(&jd.data.description.value, false),
            name: jd.name,
            alignment: jd.data.alignment.own,
            follower_alignments: jd.data.alignment.follower,
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
            "data": {
                "description": {
                    "value": "Testing"
                },
                "alignment": {
                    "follower": [
                        "LE",
                        "NE"
                    ],
                    "own": "LE"
                },
            },
            "name": "Tester"
        });

        let deity: Deity = serde_json::from_value(json).unwrap();
        assert_eq!(
            deity,
            Deity {
                name: "Tester".into(),
                content: "Testing".into(),
                alignment: Alignment::LE,
                follower_alignments: vec![Alignment::LE, Alignment::NE],
            }
        );
    }

    #[test]
    fn should_deserialize_real_deity() {
        let asmodeus: Deity = serde_json::from_str(&read_test_file("deities.db/asmodeus.json")).expect("Deserialization failed");
        assert_eq!(asmodeus.name, String::from("Asmodeus"));
    }
}
