use super::{Publication, ValueWrapper, traits::Traits};
use crate::{data::traits::JsonTraits, text_cleanup};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(from = "JsonHeritage")]
pub struct Heritage {
    pub name: String,
    pub description: String,
    pub ancestry: Option<String>,
    pub traits: Traits,
    pub source: String,
}

impl From<JsonHeritage> for Heritage {
    fn from(jh: JsonHeritage) -> Self {
        Heritage {
            name: jh.name,
            ancestry: jh.system.ancestry.map(|a| a.name),
            description: text_cleanup(&jh.system.description.value),
            traits: jh.system.traits.into(),
            source: jh.system.publication.title,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct JsonHeritage {
    system: InnerJsonHeritage,
    name: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InnerJsonHeritage {
    description: ValueWrapper<String>,
    traits: JsonTraits,
    publication: Publication,
    ancestry: Option<JsonHeritageAncestry>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct JsonHeritageAncestry {
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::traits::Rarity;
    use crate::tests::read_test_file;

    #[test]
    fn should_deserialize_heritage_with_ancestry() {
        let aquatic_elf: Heritage =
            serde_json::from_str(&read_test_file("heritages/elf/aquatic-elf.json")).expect("Deserialization failed");
        assert_eq!(aquatic_elf.name, String::from("Aquatic Elf"));
        assert_eq!(
            aquatic_elf.traits,
            Traits {
                misc: vec!["amphibious".into()],
                rarity: Rarity::Common,
                size: None,
                alignment: None,
            }
        );
        assert_eq!(aquatic_elf.source, "Pathfinder Lost Omens High Seas");
        assert_eq!(aquatic_elf.ancestry, Some(String::from("Elf")));
    }

    #[test]
    fn should_deserialize_versatile_heritage() {
        let nephilim: Heritage =
            serde_json::from_str(&read_test_file("heritages/versatile-heritages/nephilim.json")).expect("Deserialization failed");
        assert_eq!(nephilim.name, String::from("Nephilim"));
        assert_eq!(
            nephilim.traits,
            Traits {
                misc: vec!["nephilim".into()],
                rarity: Rarity::Uncommon,
                size: None,
                alignment: None,
            }
        );
        assert_eq!(nephilim.source, "Pathfinder Player Core");
        assert_eq!(nephilim.ancestry, None);
    }
}
