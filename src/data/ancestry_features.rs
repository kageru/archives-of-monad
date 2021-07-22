use crate::data::traits::Traits;
use crate::data::ValueWrapper;
use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, PartialEq)]
enum FeatType {
    Heritage,
    AncestryFeature,
}

impl<'de> Deserialize<'de> for FeatType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "heritage" => Ok(FeatType::Heritage),
            "ancestryfeature" => Ok(FeatType::AncestryFeature),
            s => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(s),
                &"heritage|ancestryfeature",
            )),
        }
    }
}

impl fmt::Display for FeatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeatType::AncestryFeature => write!(f, "AncestryFeature"),
            FeatType::Heritage => write!(f, "Heritage"),
        }
    }
}

#[derive(Deserialize)]
pub struct JsonAncestryFeature {
    data: AncestryFeatureData,
    name: String,
}

#[derive(Deserialize)]
pub struct AncestryFeatureData {
    description: ValueWrapper<String>,
    #[serde(rename = "featType")]
    feat_type: ValueWrapper<FeatType>,
    traits: Traits,
}

#[derive(Debug)]
pub struct AncestryFeature {
    name: String,
    description: String,
    feat_type: FeatType,
    traits: Traits,
}

impl From<JsonAncestryFeature> for AncestryFeature {
    fn from(jaf: JsonAncestryFeature) -> Self {
        AncestryFeature {
            name: jaf.name,
            description: jaf.data.description.value,
            feat_type: jaf.data.feat_type.value,
            traits: jaf.data.traits,
        }
    }
}

impl fmt::Display for AncestryFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}, {}", self.name, self.feat_type, self.description)
    }
}
