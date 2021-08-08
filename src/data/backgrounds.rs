use super::{HasName, ValueWrapper, ability_scores::{AbilityBoost, JsonAbilityBoosts}, skills::Skill, traits::{JsonTraits, Traits}};
use askama::Template;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, PartialEq, Debug, Template, Clone)]
#[template(path = "background.html", escape = "none")]
#[serde(from = "JsonBackground")]
pub struct Background {
    pub name: String,
    pub boosts: Vec<AbilityBoost>,
    pub description: String,
    pub feats: Vec<String>,
    pub lore: String,
    pub skills: Vec<Skill>,
    pub traits: Traits,
}

impl HasName for Background {
    fn name(&self) -> &str {
        &self.name
    }
}

impl From<JsonBackground> for Background {
    fn from(jb: JsonBackground) -> Self {
        Background {
            name: jb.name,
            boosts: jb.data.boosts.into(),
            description: jb.data.description.value,
            feats: jb.data.items.into_values().map(|i| i.name).collect(),
            lore: jb.data.trained_lore,
            skills: jb.data.trained_skills.value,
            traits: jb.data.traits.into(),
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
struct JsonBackground {
    name: String,
    data: JsonBackgroundData,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct JsonBackgroundData {
    boosts: JsonAbilityBoosts,
    description: ValueWrapper<String>,
    items: HashMap<String, JsonFeatReference>,
    trained_lore: String,
    trained_skills: ValueWrapper<Vec<Skill>>,
    traits: JsonTraits,
}

#[derive(Deserialize, PartialEq, Debug)]
struct JsonFeatReference {
    name: String,
}

#[cfg(test)]
mod tests {
    use crate::data::{ability_scores::AbilityScore, traits::Rarity};

    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_field_medic_deserialization() {
        let f = std::fs::File::open("tests/data/backgrounds/field-medic.json").expect("File missing");
        let reader = BufReader::new(f);
        let field_medic = serde_json::from_reader::<_, Background>(reader).expect("Deserialization failed");
        assert_eq!(field_medic.name.as_str(), "Field Medic");
        assert_eq!(
            field_medic.boosts.first(),
            Some(&AbilityBoost(vec![AbilityScore::Constitution, AbilityScore::Wisdom]))
        );
        assert!(field_medic.boosts[1].is_free());
        assert_eq!(field_medic.traits.rarity, Some(Rarity::Common));
        assert_eq!(field_medic.feats, vec![String::from("Battle Medicine")]);
    }

    #[test]
    fn test_haunted_deserialization() {
        let f = std::fs::File::open("tests/data/backgrounds/haunted.json").expect("File missing");
        let reader = BufReader::new(f);
        let haunted = serde_json::from_reader::<_, Background>(reader).expect("Deserialization failed");
        assert_eq!(haunted.name.as_str(), "Haunted");
        assert_eq!(haunted.traits.rarity, Some(Rarity::Rare));
        assert_eq!(haunted.skills, vec![Skill::Occultism]);
    }
}
