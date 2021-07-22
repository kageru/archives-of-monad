use crate::data::traits::{JsonTraits, Traits};
use crate::data::ValueWrapper;
use serde::{Deserialize, Deserializer};
use serde_json::json;

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
    traits: JsonTraits,
}

#[derive(Debug, PartialEq)]
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
            traits: Traits::from(jaf.data.traits),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::traits::Rarity;

    #[test]
    fn should_deserialize_action() {
        let json = json!(
        {
            "data": {
                "description": {
                    "value": "Test"
                },
                "featType": {
                    "value": "heritage"
                },
                "traits": {
                    "rarity": {
                        "value": "uncommon"
                    },
                    "value": [
                        "aasimar",
                        "versatile heritage"
                    ]
                }
            },
            "name": "Aasimar"
        })
        .to_string();
        let feature: AncestryFeature = AncestryFeature::from(serde_json::from_str::<JsonAncestryFeature>(&json).unwrap());
        assert_eq!(
            feature,
            AncestryFeature {
                name: "Aasimar".into(),
                description: "Test".into(),
                feat_type: FeatType::Heritage,
                traits: Traits {
                    value: vec!["aasimar".into(), "versatile heritage".into()],
                    rarity: Some(Rarity::Uncommon)
                },
            }
        );
    }
}
