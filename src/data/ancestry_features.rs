use crate::data::feat_type::FeatType;
use crate::data::traits::{JsonTraits, Traits};
use crate::data::ValueWrapper;
use crate::text_cleanup;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct JsonAncestryFeature {
    system: AncestryFeatureData,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AncestryFeatureData {
    description: ValueWrapper<String>,
    category: FeatType,
    traits: JsonTraits,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(from = "JsonAncestryFeature")]
pub struct AncestryFeature {
    pub name: String,
    pub description: String,
    pub feat_type: FeatType,
    pub traits: Traits,
}

impl From<JsonAncestryFeature> for AncestryFeature {
    fn from(jaf: JsonAncestryFeature) -> Self {
        AncestryFeature {
            name: jaf.name.clone(),
            description: text_cleanup(&jaf.system.description.value),
            feat_type: jaf.system.category,
            traits: Traits::from(jaf.system.traits),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{data::traits::Rarity, tests::read_test_file};

    #[test]
    fn should_deserialize_real_ancestry_feature() {
        let adaptive_anadi: AncestryFeature =
            serde_json::from_str(&read_test_file("ancestryfeatures/change-shape-anadi.json")).expect("Deserialization failed");
        assert_eq!(adaptive_anadi.name, "Change Shape (Anadi)");
        assert_eq!(adaptive_anadi.feat_type, FeatType::AncestryFeature);
        assert_eq!(
            adaptive_anadi.traits,
            Traits {
                misc: vec!["anadi".into()],
                rarity: Rarity::Common,
                size: None,
                alignment: None,
            }
        );
    }
}
