use crate::data::ValueWrapper;
use crate::data::feat_type::FeatType;
use crate::text_cleanup;
use serde::{Deserialize, Serialize, de::IgnoredAny};

#[derive(Deserialize)]
pub struct JsonBoonOrCurse {
    system: BoonOrCurseData,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoonOrCurseData {
    description: ValueWrapper<String>,
    #[serde(rename = "category")]
    feat_type: FeatType,
}

// The boons-and-curses folder also ships a handful of "effect" documents (the mechanical rule
// grants behind a boon/curse) alongside the actual boon/curse feats; those have no `category`, so
// they don't match `JsonBoonOrCurse` and fall through to the ignored catch-all instead.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum JsonBoonOrCurseDoc {
    Real(JsonBoonOrCurse),
    Other(IgnoredAny),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(from = "JsonBoonOrCurseDoc")]
pub struct BoonOrCurse {
    pub name: String,
    pub description: String,
    pub feat_type: FeatType,
}

impl From<JsonBoonOrCurseDoc> for BoonOrCurse {
    fn from(doc: JsonBoonOrCurseDoc) -> Self {
        match doc {
            JsonBoonOrCurseDoc::Real(jbc) => BoonOrCurse {
                name: jbc.name,
                description: text_cleanup(&jbc.system.description.value),
                feat_type: jbc.system.feat_type,
            },
            JsonBoonOrCurseDoc::Other(_) => BoonOrCurse {
                name: String::from("[Empty]"),
                description: String::new(),
                feat_type: FeatType::Boon,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::read_test_file;

    #[test]
    fn should_deserialize_real_boon() {
        let boon: BoonOrCurse =
            serde_json::from_str(&read_test_file("boons-and-curses/asmodeus-major-boon.json")).expect("Deserialization failed");
        assert_eq!(boon.name, String::from("Asmodeus - Major Boon"));
        assert_eq!(boon.feat_type, FeatType::Boon);
    }

    #[test]
    fn should_deserialize_real_curse() {
        let curse: BoonOrCurse =
            serde_json::from_str(&read_test_file("boons-and-curses/cayden-cailean-minor-curse.json")).expect("Deserialization failed");
        assert_eq!(curse.name, String::from("Cayden Cailean - Minor Curse"));
        assert_eq!(curse.feat_type, FeatType::Curse);
    }
}
