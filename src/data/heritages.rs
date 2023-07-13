use super::{traits::Traits, ValueWrapper};
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
            source: jh.system.source.value,
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
    source: ValueWrapper<String>,
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
        let half_elf: Heritage = serde_json::from_str(&read_test_file("heritages/half-elf.json")).expect("Deserialization failed");
        assert_eq!(half_elf.name, String::from("Half-Elf"));
        assert_eq!(
            half_elf.traits,
            Traits {
                misc: vec!["half-elf".into()],
                rarity: Rarity::Common,
                size: None,
                alignment: None,
            }
        );
        assert_eq!(half_elf.source, "Pathfinder Core Rulebook");
        assert_eq!(half_elf.ancestry, Some(String::from("Human")));
    }

    #[test]
    fn should_deserialize_versatile_heritage() {
        let aasimar: Heritage = serde_json::from_str(&read_test_file("heritages/aasimar.json")).expect("Deserialization failed");
        assert_eq!(aasimar.name, String::from("Aasimar"));
        assert_eq!(
            aasimar.traits,
            Traits {
                misc: vec!["aasimar".into()],
                rarity: Rarity::Uncommon,
                size: None,
                alignment: None,
            }
        );
        assert_eq!(aasimar.source, "Pathfinder Advanced Player's Guide");
        assert_eq!(aasimar.ancestry, None);
    }
}
