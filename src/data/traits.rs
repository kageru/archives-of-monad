use super::ValueWrapper;
use serde::{de, Deserialize, Deserializer};

#[derive(Deserialize, Debug, PartialEq)]
pub struct Traits {
    value: Vec<String>,
    rarity: Option<ValueWrapper<Rarity>>,
}

#[derive(Debug, PartialEq)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
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
