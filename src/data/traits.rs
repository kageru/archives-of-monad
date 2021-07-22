use super::ValueWrapper;
use serde::{de, Deserialize, Deserializer};

#[derive(Deserialize, Debug, PartialEq)]
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
        let rarity = match jt.rarity {
            None => None,
            Some(r) => Some(r.value),
        };
        Traits { value: jt.value, rarity }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Unique,
}

impl<'de> Deserialize<'de> for Rarity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "common" => Ok(Rarity::Common),
            "uncommon" => Ok(Rarity::Uncommon),
            "rare" => Ok(Rarity::Rare),
            "unique" => Ok(Rarity::Unique),
            s => Err(de::Error::invalid_value(de::Unexpected::Str(s), &"common|uncommon|rare")),
        }
    }
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
