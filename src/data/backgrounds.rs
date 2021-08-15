use super::{
    ability_scores::{AbilityBoost, JsonAbilityBoosts},
    skills::Skill,
    traits::{JsonTraits, Traits},
    HasName, ValueWrapper,
};
use crate::replace_references;
use itertools::Itertools;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, PartialEq, Debug, Clone)]
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

impl Background {
    pub fn condensed(&self) -> String {
        let feats = self.feats.join(", ");
        let skills = self.skills.iter().map(Skill::to_string).join(", ");
        format!(
            "Boost(s): {}; Skill(s): {}; Lore: {}; Feat: {}.",
            self.boosts.iter().map(AbilityBoost::to_string).join(", "),
            if self.skills.is_empty() { "none" } else { &skills },
            if self.lore.is_empty() { "none" } else { &self.lore },
            if self.feats.is_empty() { "none" } else { &feats },
        )
    }
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
            description: replace_references(&jb.data.description.value),
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
    use super::*;
    use crate::{
        data::{ability_scores::AbilityScore, traits::Rarity},
        tests::read_test_file,
    };

    #[test]
    fn test_field_medic_deserialization() {
        let field_medic: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/field-medic.json")).expect("Deserialization failed");
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
        let haunted: Background = serde_json::from_str(&read_test_file("backgrounds.db/haunted.json")).expect("Deserialization failed");
        assert_eq!(haunted.name.as_str(), "Haunted");
        assert_eq!(haunted.traits.rarity, Some(Rarity::Rare));
        assert_eq!(haunted.skills, vec![Skill::Occultism]);
    }
}
