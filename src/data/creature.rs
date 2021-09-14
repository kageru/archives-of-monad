use super::{
    damage::{CreatureDamage, DamageType},
    equipment::StringOrNum,
    size::Size,
    skills::Skill,
    traits::{JsonTraits, Rarity},
    HasLevel, ValueWrapper,
};
use crate::data::traits::Traits;
use convert_case::{Case, Casing};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, convert::TryFrom};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(from = "JsonCreature")]
pub struct Creature {
    pub name: String,
    pub ability_scores: AbilityModifiers,
    pub ac: i32,
    pub ac_details: Option<String>,
    pub hp: i32,
    pub hp_details: Option<String>,
    pub perception: i32,
    pub senses: String,
    pub speeds: CreatureSpeeds,
    pub flavor_text: String,
    pub level: i32,
    pub source: String,
    pub saves: SavingThrows,
    pub traits: Traits,
    pub resistances: Vec<(String, Option<i32>)>,
    pub weaknesses: Vec<(String, Option<i32>)>,
    pub immunities: Vec<String>,
    pub languages: Vec<String>,
    pub attacks: Vec<Attack>,
    pub skills: Vec<(Skill, i32)>,
}

#[derive(Serialize, PartialEq, Debug, Clone, Eq)]
pub struct Attack {
    pub damage: Vec<CreatureDamage>,
    pub modifier: i32,
    pub traits: Traits,
    pub name: String,
}

impl From<JsonCreature> for Creature {
    fn from(jc: JsonCreature) -> Self {
        let mut attacks = Vec::new();
        let mut skills = Vec::new();

        for item in jc.items {
            match item.item_type {
                CreatureItemType::Melee | CreatureItemType::Weapon => {
                    attacks.push(Attack {
                        modifier: item.data.bonus.expect("this should have a bonus").value,
                        name: item.name.clone(),
                        damage: item
                            .data
                            .damage_rolls
                            .into_values()
                            .filter_map(|dmg| CreatureDamage::try_from(dmg).ok())
                            .collect(),
                        traits: item.data.traits.clone().into(),
                    });
                }
                CreatureItemType::Lore => {
                    let skill = Skill::iter().find(|s| s.as_ref() == item.name).unwrap_or(Skill::Lore(item.name));
                    skills.push((skill, item.data.bonus.expect("this should have a bonus").value));
                }
                _ => (),
            }
        }
        Creature {
            name: jc.name,
            ability_scores: jc.data.abilities.into(),
            ac: jc.data.attributes.ac.value,
            ac_details: remove_parentheses(jc.data.attributes.ac.details),
            hp: jc.data.attributes.hp.value,
            hp_details: remove_parentheses(jc.data.attributes.hp.details),
            perception: jc.data.attributes.perception.value,
            senses: match jc.data.traits.senses {
                StringWrapperOrList::Wrapper(w) => w.value,
                StringWrapperOrList::List(l) => l.join(", "),
            },
            speeds: jc.data.attributes.speed.ensure_trailing_unit(),
            flavor_text: jc.data.details.flavor_text,
            level: jc.data.details.level.value,
            source: jc.data.details.source.value,
            saves: SavingThrows {
                reflex: jc.data.saves.reflex.value,
                fortitude: jc.data.saves.fortitude.value,
                will: jc.data.saves.will.value,
                additional_save_modifier: jc.data.attributes.all_saves.and_then(|v| v.value),
            },
            traits: Traits {
                misc: titlecased(&jc.data.traits.traits.value),
                rarity: jc.data.traits.rarity.value,
                size: Some(jc.data.traits.size.value),
                alignment: Some(jc.data.details.alignment.value),
            },
            resistances: jc.data.traits.dr.iter().map_into().collect(),
            weaknesses: jc.data.traits.dv.iter().map_into().collect(),
            immunities: lowercased(&jc.data.traits.di.value),
            languages: {
                let mut titlecased = titlecased(&jc.data.traits.languages.value);
                if !jc.data.traits.languages.custom.is_empty() {
                    titlecased.push(jc.data.traits.languages.custom.from_case(Case::Lower).to_case(Case::Title));
                }
                titlecased
            },
            attacks,
            skills,
        }
    }
}

impl From<&JsonResistanceOrWeakness> for (String, Option<i32>) {
    fn from(dr: &JsonResistanceOrWeakness) -> Self {
        (
            dr.damage_type.from_case(Case::Kebab).to_case(Case::Title),
            dr.value.as_ref().map(i32::from),
        )
    }
}

fn remove_parentheses(s: String) -> Option<String> {
    Some(s.trim_start_matches('(').trim_end_matches(')').to_string()).filter(|d| !d.is_empty())
}

fn ensure_trailing_unit(speed: &str) -> String {
    let speed = speed.trim();
    if speed.ends_with(" feet") {
        speed.to_string()
    } else {
        format!("{} feet", speed)
    }
}

fn titlecased(xs: &[String]) -> Vec<String> {
    xs.iter()
        .filter(|&l| l != "custom")
        .map(|l| l.from_case(Case::Kebab).to_case(Case::Title))
        .collect()
}

fn lowercased(xs: &[String]) -> Vec<String> {
    xs.iter()
        .filter(|&l| l != "custom")
        .map(|l| l.from_case(Case::Kebab).to_case(Case::Lower))
        .collect()
}

impl HasLevel for Creature {
    fn level(&self) -> i32 {
        self.level
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, AsRefStr, Clone, Copy)]
pub enum Alignment {
    LG,
    NG,
    CG,
    LN,
    N,
    CN,
    LE,
    NE,
    CE,
    // summons like an unseen servant are unaligned
    #[serde(other)]
    Unaligned,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, Eq)]
pub struct AbilityModifiers {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

impl From<JsonCreatureAbilities> for AbilityModifiers {
    fn from(ja: JsonCreatureAbilities) -> Self {
        Self {
            strength: ja.str.modifier,
            dexterity: ja.dex.modifier,
            constitution: ja.con.modifier,
            intelligence: ja.int.modifier,
            wisdom: ja.wis.modifier,
            charisma: ja.cha.modifier,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
pub struct SavingThrows {
    pub reflex: i32,
    pub fortitude: i32,
    pub will: i32,
    pub additional_save_modifier: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
pub struct Speeds {
    pub general: String,
    pub fly: Option<i32>,
    pub swim: Option<i32>,
    pub burrow: Option<i32>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreature {
    data: JsonCreatureData,
    name: String,
    items: Vec<JsonCreatureItem>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreatureData {
    abilities: JsonCreatureAbilities,
    attributes: JsonCreatureAttributes,
    details: JsonCreatureDetails,
    saves: JsonCreatureSaves,
    traits: JsonCreatureTraits, // different from usual traits
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreatureAbilities {
    cha: JsonCreatureAbility,
    con: JsonCreatureAbility,
    dex: JsonCreatureAbility,
    int: JsonCreatureAbility,
    str: JsonCreatureAbility,
    wis: JsonCreatureAbility,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreatureAbility {
    #[serde(rename = "mod")]
    modifier: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JsonCreatureAttributes {
    ac: AcHpDetails,
    all_saves: Option<ValueWrapper<Option<String>>>,
    hp: AcHpDetails,
    perception: ValueWrapper<i32>,
    speed: CreatureSpeeds,
}

#[derive(Deserialize, Debug, PartialEq)]
struct AcHpDetails {
    value: i32,
    details: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatureSpeeds {
    pub value: String,
    pub other_speeds: Vec<OtherCreatureSpeed>,
}

impl CreatureSpeeds {
    fn ensure_trailing_unit(self) -> Self {
        CreatureSpeeds {
            value: ensure_trailing_unit(&self.value),
            other_speeds: self
                .other_speeds
                .into_iter()
                .map(|speed| OtherCreatureSpeed {
                    speed_type: speed.speed_type,
                    value: ensure_trailing_unit(&speed.value),
                })
                .collect(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct OtherCreatureSpeed {
    #[serde(rename = "type")]
    pub speed_type: String,
    pub value: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JsonCreatureDetails {
    alignment: ValueWrapper<Alignment>,
    flavor_text: String,
    level: ValueWrapper<i32>,
    source: ValueWrapper<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreatureSaves {
    fortitude: ValueWrapper<i32>,
    reflex: ValueWrapper<i32>,
    will: ValueWrapper<i32>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreatureTraits {
    rarity: ValueWrapper<Rarity>,
    senses: StringWrapperOrList,
    size: ValueWrapper<Size>,
    // yes, traits inside the traits. amazing, I know
    traits: ValueWrapper<Vec<String>>,
    languages: JsonLanguages,
    // I think this means damage immunities, but there are sometimes conditions in it.
    // There’s also ci which I assume would be where condition immunities actually belong.
    di: ValueWrapper<Vec<String>>,
    dv: Vec<JsonResistanceOrWeakness>,
    dr: Vec<JsonResistanceOrWeakness>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonLanguages {
    custom: String,
    value: Vec<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum StringWrapperOrList {
    Wrapper(ValueWrapper<String>),
    List(Vec<String>),
}

#[derive(Deserialize, PartialEq, Debug)]
struct JsonResistanceOrWeakness {
    #[serde(rename = "type")]
    damage_type: String,
    value: Option<StringOrNum>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreatureItem {
    data: JsonCreatureItemData,
    #[serde(rename = "type")]
    item_type: CreatureItemType,
    name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JsonCreatureItemData {
    #[serde(alias = "mod")]
    bonus: Option<ValueWrapper<i32>>,
    traits: JsonTraits,
    #[serde(default)]
    damage_rolls: BTreeMap<String, JsonCreatureDamage>,
    #[serde(default)]
    attack_effects: ValueWrapper<Vec<String>>,
    // range?
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JsonCreatureDamage {
    pub damage: String,
    pub damage_type: String,
}
impl TryFrom<JsonCreatureDamage> for CreatureDamage {
    type Error = ();
    fn try_from(value: JsonCreatureDamage) -> Result<Self, Self::Error> {
        DamageType::from_str_lower(&value.damage_type)
            .map(|damage_type| CreatureDamage {
                damage: value.damage,
                damage_type,
            })
            .ok_or(())
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
enum CreatureItemType {
    Melee,
    Action,
    Lore,
    Spell,
    // combine with spell?
    SpellcastingEntry,
    Equipment,
    // is this like melee?
    Weapon,
    Consumable,
    Condition,
    Armor,
    Effect,
    Treasure,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{data::damage::DamageType, tests::read_test_file};

    #[test]
    fn test_deserialize_budget_dahak() {
        let dargon: Creature =
            serde_json::from_str(&read_test_file("pathfinder-bestiary.db/ancient-red-dragon.json")).expect("deserialization failed");
        assert_eq!(
            dargon.saves,
            SavingThrows {
                reflex: 32,
                fortitude: 35,
                will: 35,
                additional_save_modifier: Some("+1 status to all saves vs. magic".to_string()),
            }
        );
        assert_eq!(dargon.name, "Ancient Red Dragon".to_string());
        assert_eq!(dargon.perception, 35);
        assert_eq!(dargon.ac, 45);
        assert_eq!(dargon.hp, 425);
        assert_eq!(
            dargon.ability_scores,
            AbilityModifiers {
                strength: 9,
                dexterity: 5,
                constitution: 8,
                intelligence: 5,
                wisdom: 6,
                charisma: 7,
            }
        );
        assert_eq!(
            dargon.traits,
            Traits {
                misc: vec!["Dragon".to_string(), "Fire".to_string()],
                size: Some(Size::Huge),
                alignment: Some(Alignment::CE),
                rarity: Rarity::Uncommon,
            }
        );
        assert_eq!(dargon.senses, "darkvision, scent (imprecise) 60 feet, smoke vision");
        assert_eq!(dargon.weaknesses, vec![("Cold".to_string(), Some(20))]);
        assert_eq!(
            dargon.immunities,
            vec!["fire".to_string(), "paralyzed".to_string(), "sleep".to_string()]
        );
        assert_eq!(
            dargon.languages,
            vec![
                "Abyssal".to_string(),
                "Common".to_string(),
                "Draconic".to_string(),
                "Dwarven".to_string(),
                "Jotun".to_string(),
                "Orcish".to_string(),
            ]
        );
        assert_eq!(
            dargon.attacks,
            vec![
                Attack {
                    damage: vec![
                        CreatureDamage {
                            damage: "4d10+17".to_string(),
                            damage_type: DamageType::Piercing
                        },
                        CreatureDamage {
                            damage: "3d6".to_string(),
                            damage_type: DamageType::Fire
                        },
                    ],
                    modifier: 37,
                    traits: Traits {
                        misc: vec!["fire".to_string(), "magical".to_string(), "reach-20".to_string()],
                        rarity: Rarity::Common,
                        alignment: None,
                        size: None
                    },
                    name: "Jaws".to_string(),
                },
                Attack {
                    damage: vec![CreatureDamage {
                        damage: "4d8+17".to_string(),
                        damage_type: DamageType::Slashing
                    }],
                    modifier: 37,
                    traits: Traits {
                        misc: vec!["agile".to_string(), "magical".to_string(), "reach-15".to_string()],
                        rarity: Rarity::Common,
                        alignment: None,
                        size: None
                    },
                    name: "Claw".to_string(),
                },
                Attack {
                    damage: vec![CreatureDamage {
                        damage: "4d10+15".to_string(),
                        damage_type: DamageType::Slashing
                    }],
                    modifier: 35,
                    traits: Traits {
                        misc: vec!["magical".to_string(), "reach-25".to_string()],
                        rarity: Rarity::Common,
                        alignment: None,
                        size: None
                    },
                    name: "Tail".to_string(),
                },
                Attack {
                    damage: vec![CreatureDamage {
                        damage: "3d8+15".to_string(),
                        damage_type: DamageType::Slashing
                    }],
                    modifier: 35,
                    traits: Traits {
                        misc: vec!["agile".to_string(), "magical".to_string(), "reach-20".to_string()],
                        rarity: Rarity::Common,
                        alignment: None,
                        size: None
                    },
                    name: "Wing".to_string(),
                }
            ]
        );
        assert_eq!(
            dargon.skills,
            vec![
                (Skill::Acrobatics, 30),
                (Skill::Arcana, 35),
                (Skill::Athletics, 37),
                (Skill::Deception, 35),
                (Skill::Diplomacy, 35),
                (Skill::Intimidation, 37),
                (Skill::Stealth, 33),
            ]
        );
    }
}
