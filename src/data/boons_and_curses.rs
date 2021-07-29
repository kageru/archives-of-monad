use crate::data::feat_type::FeatType;
use crate::data::ValueWrapper;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonBoonOrCurse {
    data: BoonOrCurseData,
    name: String,
}

#[derive(Deserialize)]
pub struct BoonOrCurseData {
    description: ValueWrapper<String>,
    #[serde(rename = "featType")]
    feat_type: ValueWrapper<FeatType>,
}

#[derive(Debug, PartialEq)]
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
    use std::io::BufReader;

    #[test]
    fn should_deserialize_real_boon() {
        let f = std::fs::File::open("tests/data/boons_and_curses/asmodeus-major-boon.json").expect("File missing");
        let reader = BufReader::new(f);
        let boon: JsonBoonOrCurse = serde_json::from_reader(reader).expect("Deserialization failed");
        let boon = BoonOrCurse::from(boon);
        assert_eq!(boon.name, String::from("Asmodeus - Major Boon"));
        assert_eq!(boon.feat_type, FeatType::Boon);
    }

    #[test]
    fn should_deserialize_real_curse() {
        let f = std::fs::File::open("tests/data/boons_and_curses/cayden-cailean-minor-curse.json").expect("File missing");
        let reader = BufReader::new(f);
        let curse: JsonBoonOrCurse = serde_json::from_reader(reader).expect("Deserialization failed");
        let curse = BoonOrCurse::from(curse);
        assert_eq!(curse.name, String::from("Cayden Cailean - Minor Curse"));
        assert_eq!(curse.feat_type, FeatType::Curse);
    }
}
