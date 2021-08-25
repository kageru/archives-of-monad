use crate::data::feat_type::FeatType;
use crate::data::traits::{JsonTraits, Traits};
use crate::data::ValueWrapper;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct JsonAncestryFeature {
    data: AncestryFeatureData,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AncestryFeatureData {
    description: ValueWrapper<String>,
    feat_type: ValueWrapper<FeatType>,
    traits: JsonTraits,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(from = "JsonAncestryFeature")]
pub struct AncestryFeature {
    name: String,
    description: String,
    feat_type: FeatType,
    traits: Traits,
}

impl From<JsonAncestryFeature> for AncestryFeature {
    fn from(jaf: JsonAncestryFeature) -> Self {
        AncestryFeature {
            name: jaf.name.clone(),
            description: jaf.data.description.value,
            feat_type: jaf.data.feat_type.value,
            traits: Traits::from(jaf.data.traits),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{data::traits::Rarity, tests::read_test_file};
    use serde_json::json;

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
        });
        let feature: AncestryFeature = serde_json::from_value(json).unwrap();
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

    #[test]
    fn should_deserialize_real_ancestry_feature() {
        let adaptive_anadi: AncestryFeature =
            serde_json::from_str(&read_test_file("ancestryfeatures.db/adaptive-anadi.json")).expect("Deserialization failed");
        assert_eq!(adaptive_anadi.name, String::from("Adaptive Anadi"));
        assert_eq!(adaptive_anadi.feat_type, FeatType::Heritage);
        assert_eq!(
            adaptive_anadi.traits,
            Traits {
                value: vec!["anadi".into(), "heritage".into()],
                rarity: Some(Rarity::Common),
            }
        );
    }
}
