use super::{traits::Traits, ValueWrapper};
use crate::data::ability_scores::AbilityScore;
use crate::data::proficiency::Proficiency;
use crate::data::skills::Skill;
use crate::data::traits::JsonTraits;
use crate::data::{string_or_i32, I32Wrapper};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Class {
    name: String,
    boost_levels: Vec<i32>,
    ancestry_feat_levels: Vec<i32>,
    attacks: AttacksProficiencies,
    class_dc: Proficiency,
    class_feat_levels: Vec<i32>,
    defenses: DefensiveProficiencies,
    description: String,
    general_feat_levels: Vec<i32>,
    hp: i32,
    key_ability: Vec<AbilityScore>,
    perception: Proficiency,
    saving_throws: SavingThrowProficiencies,
    skill_feat_levels: Vec<i32>,
    skill_increase_levels: Vec<i32>,
    trained_skills: Vec<Skill>,
    free_skills: i32,
    traits: Traits,
    class_features: Vec<ClassItem>,
}

impl From<JsonClass> for Class {
    fn from(jc: JsonClass) -> Self {
        Class {
            name: jc.name,
            boost_levels: jc.data.boost_levels.value,
            ancestry_feat_levels: jc.data.ancestry_feat_levels.value,
            attacks: jc.data.attacks,
            class_dc: jc.data.class_dc,
            class_feat_levels: jc.data.class_feat_levels.value,
            defenses: jc.data.defenses,
            description: jc.data.description.value,
            general_feat_levels: jc.data.general_feat_levels.value,
            hp: jc.data.hp,
            key_ability: jc.data.key_ability.value,
            perception: jc.data.perception,
            saving_throws: jc.data.saving_throws,
            skill_feat_levels: jc.data.skill_feat_levels.value.into_iter().map(|v| v.0).collect(),
            skill_increase_levels: jc.data.skill_increase_levels.value.into_iter().map(|v| v.0).collect(),
            trained_skills: jc.data.skills.value,
            free_skills: jc.data.skills.additional,
            traits: jc.data.traits.into(),
            class_features: jc.data.class_features.into_iter().map(|(_, v)| v).collect(),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct JsonClass {
    data: InnerJsonClass,
    name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct InnerJsonClass {
    #[serde(rename = "abilityBoostLevels")]
    boost_levels: ValueWrapper<Vec<i32>>,
    #[serde(rename = "ancestryFeatLevels")]
    ancestry_feat_levels: ValueWrapper<Vec<i32>>,
    attacks: AttacksProficiencies,
    #[serde(rename = "classDC")]
    class_dc: Proficiency,
    #[serde(rename = "classFeatLevels")]
    class_feat_levels: ValueWrapper<Vec<i32>>,
    defenses: DefensiveProficiencies,
    description: ValueWrapper<String>,
    #[serde(rename = "generalFeatLevels")]
    general_feat_levels: ValueWrapper<Vec<i32>>,
    hp: i32,
    #[serde(rename = "items")]
    class_features: HashMap<String, ClassItem>,
    #[serde(rename = "keyAbility")]
    key_ability: ValueWrapper<Vec<AbilityScore>>,
    perception: Proficiency,
    #[serde(rename = "savingThrows")]
    saving_throws: SavingThrowProficiencies,
    #[serde(rename = "skillFeatLevels")]
    skill_feat_levels: ValueWrapper<Vec<I32Wrapper>>,
    #[serde(rename = "skillIncreaseLevels")]
    skill_increase_levels: ValueWrapper<Vec<I32Wrapper>>,
    #[serde(rename = "trainedSkills")]
    skills: TrainedSkills,
    traits: JsonTraits,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct AttacksProficiencies {
    unarmed: Proficiency,
    simple: Proficiency,
    martial: Proficiency,
    advanced: Proficiency,
    other: OtherAttacksProficiencies,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct OtherAttacksProficiencies {
    name: String,
    rank: Proficiency,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct DefensiveProficiencies {
    unarmored: Proficiency,
    light: Proficiency,
    medium: Proficiency,
    heavy: Proficiency,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct SavingThrowProficiencies {
    fortitude: Proficiency,
    reflex: Proficiency,
    will: Proficiency,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ClassItem {
    name: String,
    #[serde(deserialize_with = "string_or_i32")]
    level: i32,
    pack: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TrainedSkills {
    additional: i32,
    value: Vec<Skill>,
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;
    use crate::data::ability_scores::AbilityScore;
    use crate::data::traits::Rarity;
    #[test]
    fn should_deserialize_class() {
        let f = std::fs::File::open("tests/data/classes/rogue.json").expect("File missing");
        let reader = BufReader::new(f);
        let rogue: JsonClass = serde_json::from_reader(reader).expect("Deserialization failed");
        let rogue = Class::from(rogue);
        assert_eq!(rogue.name, String::from("Rogue"));
        assert_eq!(rogue.boost_levels, vec![5, 10, 15, 20]);
        assert_eq!(rogue.ancestry_feat_levels, vec![1, 5, 9, 13, 17]);
        assert_eq!(rogue.class_feat_levels, vec![1, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
        assert_eq!(rogue.general_feat_levels, vec![3, 7, 11, 15, 19]);
        assert_eq!(rogue.skill_feat_levels, (1..=20).collect::<Vec<_>>());
        assert_eq!(rogue.skill_increase_levels, (2..=20).collect::<Vec<_>>());
        assert_eq!(rogue.class_dc, Proficiency::Trained);
        assert_eq!(rogue.perception, Proficiency::Expert);
        assert_eq!(
            rogue.attacks,
            AttacksProficiencies {
                unarmed: Proficiency::Trained,
                simple: Proficiency::Trained,
                martial: Proficiency::Untrained,
                advanced: Proficiency::Untrained,
                other: OtherAttacksProficiencies {
                    name: "Rapier, Sap, Shortbow, and Shortsword".to_string(),
                    rank: Proficiency::Trained
                }
            }
        );
        assert_eq!(
            rogue.saving_throws,
            SavingThrowProficiencies {
                fortitude: Proficiency::Trained,
                reflex: Proficiency::Expert,
                will: Proficiency::Expert,
            }
        );
        assert_eq!(
            rogue.defenses,
            DefensiveProficiencies {
                unarmored: Proficiency::Trained,
                light: Proficiency::Trained,
                heavy: Proficiency::Untrained,
                medium: Proficiency::Untrained,
            }
        );
        assert_eq!(rogue.hp, 8);
        assert_eq!(
            rogue.key_ability,
            vec![
                AbilityScore::Charisma,
                AbilityScore::Dexterity,
                AbilityScore::Intelligence,
                AbilityScore::Strength
            ]
        );
        assert_eq!(rogue.trained_skills, vec![Skill::Stealth]);
        assert_eq!(rogue.free_skills, 7);

        let mut rogue_class_features = rogue.class_features.iter().map(|f| &f.name).collect::<Vec<_>>();
        rogue_class_features.sort();

        assert_eq!(
            rogue_class_features.first().unwrap().to_string(),
            "Debilitating Strikes".to_string()
        );
        assert_eq!(rogue_class_features.last().unwrap().to_string(), "Weapon Tricks".to_string());

        assert_eq!(
            rogue.traits,
            Traits {
                value: vec![],
                rarity: Some(Rarity::Common),
            }
        );
    }
}
