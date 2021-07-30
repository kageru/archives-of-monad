use super::{
    ability_scores::{AbilityBoost, JsonAbilityBoosts},
    skills::Skill,
    traits::{JsonTraits, Traits},
    ValueWrapper,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct Background {
    name: String,
    boosts: Vec<AbilityBoost>,
    description: String,
    feats: Vec<String>,
    lore: String,
    skills: Vec<Skill>,
    traits: Traits,
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
struct JsonBackgroundData {
    boosts: JsonAbilityBoosts,
    description: ValueWrapper<String>,
    items: HashMap<String, JsonFeatReference>,
    #[serde(rename = "trainedLore")]
    trained_lore: String,
    #[serde(rename = "trainedSkills")]
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
        let field_medic = Background::from(serde_json::from_reader::<_, JsonBackground>(reader).expect("Deserialization failed"));
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
        let haunted = Background::from(serde_json::from_reader::<_, JsonBackground>(reader).expect("Deserialization failed"));
        assert_eq!(haunted.name.as_str(), "Haunted");
        assert_eq!(haunted.traits.rarity, Some(Rarity::Rare));
        assert_eq!(haunted.skills, vec![Skill::Occultism]);
    }
}
