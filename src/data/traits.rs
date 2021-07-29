use super::ValueWrapper;
use crate::impl_deser;
use serde::{Deserialize, Deserializer};

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

impl_deser! {
    Rarity :
    "common" => Rarity::Common,
    "uncommon" => Rarity::Uncommon,
    "rare" => Rarity::Rare,
    "unique" => Rarity::Unique,
    expects: "common|uncommon|rare"
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
