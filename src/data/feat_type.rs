use crate::impl_deser;
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub enum FeatType {
    Heritage,
    AncestryFeature,
    ClassFeature,
    Boon,
    Curse,
}

impl_deser! {
    FeatType :
    "heritage" => FeatType::Heritage,
    "ancestryfeature" => FeatType::AncestryFeature,
    "classfeature" => FeatType::ClassFeature,
    "deityboon" => FeatType::Boon,
    "curse" => FeatType::Curse,
    expects: "heritage|ancestryfeature|classfeature|deityboon|curse"
}
