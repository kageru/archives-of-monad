use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub enum FeatType {
    Heritage,
    AncestryFeature,
    ClassFeature,
    Boon,
    Curse,
}

impl<'de> Deserialize<'de> for FeatType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "heritage" => Ok(FeatType::Heritage),
            "ancestryfeature" => Ok(FeatType::AncestryFeature),
            "classfeature" => Ok(FeatType::ClassFeature),
            "deityboon" => Ok(FeatType::Boon),
            "curse" => Ok(FeatType::Curse),
            s => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(s),
                &"heritage|ancestryfeature|classfeature|deityboon|curse",
            )),
        }
    }
}
