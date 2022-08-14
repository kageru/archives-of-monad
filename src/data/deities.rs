use super::{creature::Alignment, ValueWrapper};
use crate::text_cleanup;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(from = "JsonDeity")]
pub struct Deity {
    pub content: String,
    pub name: String,
    // Some meta deities are unaligned
    pub alignment: Option<Alignment>,
    pub follower_alignments: Vec<Alignment>,
}

#[derive(Deserialize, Debug)]
struct JsonDeity {
    name: String,
    system: JsonDeityData,
}

#[derive(Deserialize, Debug)]
struct JsonDeityData {
    description: ValueWrapper<String>,
    alignment: JsonDeityAlignment,
}

#[derive(Deserialize, Debug)]
struct JsonDeityAlignment {
    own: Option<Alignment>,
    follower: Vec<Alignment>,
}

impl From<JsonDeity> for Deity {
    fn from(jd: JsonDeity) -> Self {
        Deity {
            content: text_cleanup(&jd.system.description.value),
            name: jd.name,
            alignment: jd.system.alignment.own,
            follower_alignments: jd.system.alignment.follower,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::read_test_file;

    #[test]
    fn should_deserialize_real_deity() {
        let asmodeus: Deity = serde_json::from_str(&read_test_file("deities.db/asmodeus.json")).expect("Deserialization failed");
        assert_eq!(asmodeus.name, String::from("Asmodeus"));
    }
}
