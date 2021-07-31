use super::ValueWrapper;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(from = "JsonTraits")]
pub struct Traits {
    pub value: Vec<String>,
    pub rarity: Option<Rarity>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct JsonTraits {
    pub value: Vec<String>,
    pub rarity: Option<ValueWrapper<Rarity>>,
}

impl From<JsonTraits> for Traits {
    fn from(jt: JsonTraits) -> Self {
        let rarity = jt.rarity.map(|r| r.value);
        Traits { value: jt.value, rarity }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Unique,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::ValueWrapper;

    #[test]
    fn should_deserialize_rarity() {
        let json = r#"{ "value": "rare" }"#;
        let size: ValueWrapper<Rarity> = serde_json::from_str(json).unwrap();
        assert_eq!(size.value, Rarity::Rare);
    }
}
