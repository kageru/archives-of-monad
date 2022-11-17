use crate::data::{
    ability_scores::AbilityScore,
    class_features::LEVEL_ANNOTATION,
    equipment::StringOrNum,
    proficiency::Proficiency,
    skills::Skill,
    traits::{JsonTraits, Traits},
    ValueWrapper,
};
use crate::text_cleanup;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(from = "JsonClass")]
pub struct Class {
    pub name: String,
    pub ancestry_feat_levels: Vec<i32>,
    pub attacks: AttackProficiencies,
    pub class_dc: Proficiency,
    pub class_feat_levels: Vec<i32>,
    pub defenses: DefensiveProficiencies,
    pub description: String,
    pub general_feat_levels: Vec<i32>,
    pub hp: i32,
    pub key_ability: Vec<AbilityScore>,
    pub perception: Proficiency,
    pub saving_throws: SavingThrowProficiencies,
    pub skill_feat_levels: Vec<i32>,
    pub skill_increase_levels: Vec<i32>,
    pub trained_skills: Vec<Skill>,
    pub free_skills: i32,
    pub traits: Traits,
    pub class_features: Vec<ClassItem>,
}

impl From<JsonClass> for Class {
    fn from(jc: JsonClass) -> Self {
        Class {
            name: jc.name.clone(),
            ancestry_feat_levels: jc.system.ancestry_feat_levels.value,
            attacks: jc.system.attacks,
            class_dc: jc.system.class_dc,
            class_feat_levels: jc.system.class_feat_levels.value,
            defenses: jc.system.defenses,
            description: text_cleanup(&jc.system.description.value),
            general_feat_levels: jc.system.general_feat_levels.value,
            hp: jc.system.hp,
            key_ability: jc.system.key_ability.value,
            perception: jc.system.perception,
            saving_throws: jc.system.saving_throws,
            skill_feat_levels: jc.system.skill_feat_levels.value,
            skill_increase_levels: jc.system.skill_increase_levels.value,
            trained_skills: jc.system.trained_skills.value,
            free_skills: jc.system.trained_skills.additional,
            traits: jc.system.traits.into(),
            class_features: jc.system.class_features.into_values().map_into().collect(),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct JsonClass {
    system: InnerJsonClass,
    name: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InnerJsonClass {
    ancestry_feat_levels: ValueWrapper<Vec<i32>>,
    attacks: AttackProficiencies,
    #[serde(rename = "classDC")]
    class_dc: Proficiency,
    class_feat_levels: ValueWrapper<Vec<i32>>,
    defenses: DefensiveProficiencies,
    description: ValueWrapper<String>,
    general_feat_levels: ValueWrapper<Vec<i32>>,
    hp: i32,
    #[serde(rename = "items")]
    class_features: HashMap<String, JsonClassItem>,
    key_ability: ValueWrapper<Vec<AbilityScore>>,
    perception: Proficiency,
    saving_throws: SavingThrowProficiencies,
    skill_feat_levels: ValueWrapper<Vec<i32>>,
    skill_increase_levels: ValueWrapper<Vec<i32>>,
    trained_skills: TrainedSkills,
    traits: JsonTraits,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AttackProficiencies {
    pub unarmed: Proficiency,
    pub simple: Proficiency,
    pub martial: Proficiency,
    pub advanced: Proficiency,
    pub other: OtherAttacksProficiencies,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct OtherAttacksProficiencies {
    pub name: String,
    pub rank: Proficiency,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct DefensiveProficiencies {
    pub unarmored: Proficiency,
    pub light: Proficiency,
    pub medium: Proficiency,
    pub heavy: Proficiency,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct SavingThrowProficiencies {
    pub fortitude: Proficiency,
    pub reflex: Proficiency,
    pub will: Proficiency,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ClassItem {
    pub name: String,
    pub level: i32,
}

impl From<JsonClassItem> for ClassItem {
    fn from(j: JsonClassItem) -> Self {
        ClassItem {
            name: LEVEL_ANNOTATION.replace_all(&j.name, "").to_string(),
            level: j.level.into(),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct JsonClassItem {
    pub name: String,
    pub level: StringOrNum,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TrainedSkills {
    additional: i32,
    value: Vec<Skill>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::ability_scores::AbilityScore;
    use crate::data::traits::Rarity;
    use crate::tests::read_test_file;

    #[test]
    fn should_deserialize_class() {
        let rogue: Class = serde_json::from_str(&read_test_file("classes.db/rogue.json")).expect("Deserialization failed");
        assert_eq!(rogue.name, "Rogue");
        assert_eq!(rogue.ancestry_feat_levels, vec![1, 5, 9, 13, 17]);
        assert_eq!(rogue.class_feat_levels, vec![1, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20]);
        assert_eq!(rogue.general_feat_levels, vec![3, 7, 11, 15, 19]);
        assert_eq!(rogue.skill_feat_levels, (1..=20).collect::<Vec<_>>());
        assert_eq!(rogue.skill_increase_levels, (2..=20).collect::<Vec<_>>());
        assert_eq!(rogue.class_dc, Proficiency::Trained);
        assert_eq!(rogue.perception, Proficiency::Expert);
        assert_eq!(
            rogue.attacks,
            AttackProficiencies {
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
                misc: vec![],
                rarity: Rarity::Common,
                size: None,
                alignment: None,
            }
        );
    }
}
