use crate::data::feat_type::FeatType;
use crate::data::ValueWrapper;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct JsonBoonOrCurse {
    data: BoonOrCurseData,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoonOrCurseData {
    description: ValueWrapper<String>,
    feat_type: ValueWrapper<FeatType>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(from = "JsonBoonOrCurse")]
pub struct BoonOrCurse {
    name: String,
    description: String,
    feat_type: FeatType,
}

impl From<JsonBoonOrCurse> for BoonOrCurse {
    fn from(jbc: JsonBoonOrCurse) -> Self {
        BoonOrCurse {
            name: jbc.name,
            description: jbc.data.description.value,
            feat_type: jbc.data.feat_type.value,
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
            serde_json::from_str(&read_test_file("boons-and-curses.db/asmodeus-major-boon.json")).expect("Deserialization failed");
        assert_eq!(boon.name, String::from("Asmodeus - Major Boon"));
        assert_eq!(boon.feat_type, FeatType::Boon);
    }

    #[test]
    fn should_deserialize_real_curse() {
        let curse: BoonOrCurse =
            serde_json::from_str(&read_test_file("boons-and-curses.db/cayden-cailean-minor-curse.json")).expect("Deserialization failed");
        assert_eq!(curse.name, String::from("Cayden Cailean - Minor Curse"));
        assert_eq!(curse.feat_type, FeatType::Curse);
    }
}
