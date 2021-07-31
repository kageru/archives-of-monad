use crate::impl_deser;
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub enum FeatType {
    AncestryFeature,
    Heritage,
    Class,
    ClassFeature,
    Archetype,
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
    "archetype" => FeatType::Archetype,
    "class" => FeatType::Class,
    expects: "heritage|ancestryfeature|classfeature|deityboon|curse|archetype|class"
}
