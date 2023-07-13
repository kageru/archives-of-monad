use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Ancestry {
    pub name: String,
    pub description: String,
    pub traits: Vec<TraitNew>,
    pub source: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
pub struct TraitNew {
    pub name: String,
    pub url: String,
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    use crate::data::ability_scores::AbilityScore;
    use crate::data::traits::Rarity;
    use crate::tests::{read_scraped_file, read_test_file};

    #[test]
    fn should_deserialize_ancestry() {
        let ancestries: Vec<Ancestry> = serde_json::from_str(&read_scraped_file("ancestries")).expect("Deserialization failed");
        assert_eq!(ancestries[0].name, String::from("Dwarf"));
        assert_eq!(
            ancestries[0].traits,
            vec![
                TraitNew {
                    name: "Dwarf".to_string(),
                    url: "/Traits.aspx?ID=54".to_string(),
                },
                TraitNew {
                    name: "Humanoid".to_string(),
                    url: "/Traits.aspx?ID=91".to_string(),
                },
            ]
        );
        assert_eq!(ancestries[0].source, "Core Rulebook pg. 35");
    }
}
