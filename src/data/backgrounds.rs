use super::{
    ability_scores::{AbilityBoost, JsonAbilityBoosts},
    skills::Skill,
    traits::{JsonTraits, Traits},
    HasName, ValueWrapper,
};
use crate::data::ObjectName;
use crate::text_cleanup;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(from = "JsonBackground")]
pub struct Background {
    pub name: String,
    pub boosts: Vec<AbilityBoost>,
    pub description: String,
    pub feats: Vec<String>,
    pub lore: String,
    pub skills: Vec<Skill>,
    pub traits: Traits,
    pub source: String,
}

impl Background {
    pub fn condensed(&self) -> String {
        let feats = self
            .feats
            .iter()
            .map(|feat| ObjectName(feat))
            .map(|feat| format!("<a href=\"/feat/{}\">{}</a>", feat.url_name(), feat.name()))
            .join(", ");
        let skills = self.skills.iter().map_into::<&str>().join(", ");
        format!(
            "Boost(s): {}; Skill(s): {}; Lore: {}; Feat: {}",
            self.boosts.iter().map(AbilityBoost::to_string).join(", "),
            if self.skills.is_empty() { "none" } else { &skills },
            if self.lore.is_empty() { "none" } else { &self.lore },
            if self.feats.is_empty() { "none" } else { &feats },
        )
    }
}

impl From<JsonBackground> for Background {
    fn from(jb: JsonBackground) -> Self {
        Background {
            name: jb.name.clone(),
            boosts: jb.system.boosts.into(),
            description: text_cleanup(&jb.system.description.value),
            feats: jb.system.items.into_values().map(|i| i.name).collect(),
            lore: match jb.system.trained_lore.as_str() {
                "" => String::from("none"),
                lore if lore.ends_with(" Lore") => lore.replace('<', "&lt;").replace('>', "&gt;"),
                lore => format!("{} Lore", lore.replace('<', "&lt;").replace('>', "&gt;")),
            },
            skills: jb.system.trained_skills.value,
            traits: jb.system.traits.into(),
            source: jb.system.source.value,
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
struct JsonBackground {
    name: String,
    system: JsonBackgroundData,
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
    source: ValueWrapper<String>,
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
        assert_eq!(field_medic.traits.rarity, Rarity::Common);
        assert_eq!(field_medic.feats, vec![String::from("Battle Medicine")]);
        assert_eq!(field_medic.source, "Pathfinder Core Rulebook");
    }

    #[test]
    fn test_haunted_deserialization() {
        let haunted: Background = serde_json::from_str(&read_test_file("backgrounds.db/haunted.json")).expect("Deserialization failed");
        assert_eq!(haunted.name.as_str(), "Haunted");
        assert_eq!(haunted.traits.rarity, Rarity::Rare);
        assert_eq!(haunted.skills, vec![Skill::Occultism]);
    }
}
