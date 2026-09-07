use super::ValueWrapper;
use crate::text_cleanup;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(from = "JsonDeity")]
pub struct Deity {
    pub content: String,
    pub name: String,
    // The remaster replaced deity alignments with this holy/unholy sanctification system.
    pub sanctification: Option<String>,
}

#[derive(Deserialize, Debug)]
struct JsonDeity {
    name: String,
    system: JsonDeityData,
}

#[derive(Deserialize, Debug)]
struct JsonDeityData {
    description: ValueWrapper<String>,
    sanctification: Option<JsonSanctification>,
}

#[derive(Deserialize, Debug)]
struct JsonSanctification {
    modal: String,
    what: Vec<String>,
}

impl From<JsonDeity> for Deity {
    fn from(jd: JsonDeity) -> Self {
        Deity {
            content: text_cleanup(&jd.system.description.value),
            name: jd.name,
            sanctification: jd.system.sanctification.map(|s| {
                let modal = if s.modal == "must" { "Must be" } else { "Can be" };
                format!("{} {}", modal, s.what.join(" or "))
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::read_test_file;

    #[test]
    fn should_deserialize_real_deity() {
        let asmodeus: Deity = serde_json::from_str(&read_test_file("deities/core-gods/asmodeus.json")).expect("Deserialization failed");
        assert_eq!(asmodeus.name, String::from("Asmodeus"));
    }
}
