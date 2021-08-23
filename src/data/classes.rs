use super::{traits::Traits, ValueWrapper};
use crate::data::ability_scores::AbilityScore;
use crate::data::proficiency::Proficiency;
use crate::data::skills::Skill;
use crate::data::traits::JsonTraits;
use crate::data::{string_or_i32, I32Wrapper};
use crate::replace_references;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(from = "JsonClass")]
pub struct Class {
    pub name: String,
    pub boost_levels: Vec<i32>,
    pub ancestry_feat_levels: Vec<i32>,
    pub attacks: AttacksProficiencies,
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
            name: jc.name,
            boost_levels: jc.data.ability_boost_levels.value,
            ancestry_feat_levels: jc.data.ancestry_feat_levels.value,
            attacks: jc.data.attacks,
            class_dc: jc.data.class_dc,
            class_feat_levels: jc.data.class_feat_levels.value,
            defenses: jc.data.defenses,
            description: replace_references(&jc.data.description.value),
            general_feat_levels: jc.data.general_feat_levels.value,
            hp: jc.data.hp,
            key_ability: jc.data.key_ability.value,
            perception: jc.data.perception,
            saving_throws: jc.data.saving_throws,
            skill_feat_levels: jc.data.skill_feat_levels.value.into_iter().map(|v| v.0).collect(),
            skill_increase_levels: jc.data.skill_increase_levels.value.into_iter().map(|v| v.0).collect(),
            trained_skills: jc.data.trained_skills.value,
            free_skills: jc.data.trained_skills.additional,
            traits: jc.data.traits.into(),
            class_features: jc.data.class_features.into_values().collect(),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct JsonClass {
    data: InnerJsonClass,
    name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InnerJsonClass {
    ability_boost_levels: ValueWrapper<Vec<i32>>,
    ancestry_feat_levels: ValueWrapper<Vec<i32>>,
    attacks: AttacksProficiencies,
    #[serde(rename = "classDC")]
    class_dc: Proficiency,
    class_feat_levels: ValueWrapper<Vec<i32>>,
    defenses: DefensiveProficiencies,
    description: ValueWrapper<String>,
    general_feat_levels: ValueWrapper<Vec<i32>>,
    hp: i32,
    #[serde(rename = "items")]
    class_features: HashMap<String, ClassItem>,
    key_ability: ValueWrapper<Vec<AbilityScore>>,
    perception: Proficiency,
    saving_throws: SavingThrowProficiencies,
    skill_feat_levels: ValueWrapper<Vec<I32Wrapper>>,
    skill_increase_levels: ValueWrapper<Vec<I32Wrapper>>,
    trained_skills: TrainedSkills,
    traits: JsonTraits,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AttacksProficiencies {
    unarmed: Proficiency,
    simple: Proficiency,
    martial: Proficiency,
    advanced: Proficiency,
    other: OtherAttacksProficiencies,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct OtherAttacksProficiencies {
    name: String,
    rank: Proficiency,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct DefensiveProficiencies {
    unarmored: Proficiency,
    light: Proficiency,
    medium: Proficiency,
    heavy: Proficiency,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct SavingThrowProficiencies {
    fortitude: Proficiency,
    reflex: Proficiency,
    will: Proficiency,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct ClassItem {
    name: String,
    #[serde(deserialize_with = "string_or_i32")]
    level: i32,
    pack: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
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
