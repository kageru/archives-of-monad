use super::{
    actions::{Action, JsonAction},
    damage::{CreatureDamage, DamageType},
    ensure_trailing_unit,
    equipment::StringOrNum,
    size::Size,
    skills::Skill,
    spells::{JsonSpell, JsonSpellData, Spell},
    traits::{JsonTraits, Rarity},
    HasLevel, HasName, ValueWrapper,
};
use crate::data::traits::Traits;
use convert_case::{Case, Casing};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeMap, convert::TryFrom};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(from = "JsonNpc")]
pub enum Npc {
    Creature(Box<Creature>),
    Hazard(Box<Hazard>),
}

impl HasName for Npc {
    fn name(&self) -> &str {
        match self {
            Npc::Creature(c) => &c.name,
            Npc::Hazard(h) => &h.name,
        }
    }
}

impl HasLevel for Npc {
    fn level(&self) -> i32 {
        match self {
            Npc::Creature(c) => c.level,
            Npc::Hazard(h) => h.level,
        }
    }
}

impl From<JsonNpc> for Npc {
    fn from(j: JsonNpc) -> Self {
        match j {
            JsonNpc::JsonCreature(c) => Npc::Creature(Box::new(c.into())),
            JsonNpc::JsonHazard(h) => Npc::Hazard(Box::new(h.into())),
        }
    }
}

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
    pub flavor_text: Option<String>,
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
    pub spellcasting: Vec<SpellCasting>,
    pub actions: Vec<Action>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(from = "JsonHazard")]
pub struct Hazard {
    name: String,
    level: i32,
}

impl From<JsonHazard> for Hazard {
    fn from(j: JsonHazard) -> Self {
        Hazard {
            name: j.name,
            level: j.data.details.level.value,
        }
    }
}

#[derive(Serialize, PartialEq, Debug, Clone, Eq)]
pub struct SpellCasting {
    pub name: String,
    pub dc: i32,
    pub spells: Vec<Spell>,
    pub id: String,
    pub slots: BTreeMap<i32, i32>,
    pub casting_type: SpellCastingType,
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
        let mut actions = Vec::new();
        let mut attacks = Vec::new();
        let mut skills = Vec::new();
        let mut spellcasting = Vec::new();

        for item in jc.items {
            match item.item_type {
                CreatureItemType::Weapon => {
                    let name = &item.name;
                    let data: JsonCreatureItemData = serde_json::from_value(item.data)
                        .unwrap_or_else(|e| panic!("Could not deserialize item data for {}: {:?}", name, e));
                    let attack = Attack {
                        modifier: data.bonus.expect("this should have a bonus").value.into(),
                        name: item.name,
                        damage: match data.damage_rolls {
                            JsonDamageRolls::Map(m) => m.into_values().filter_map(|dmg| CreatureDamage::try_from(dmg).ok()).collect(),
                            JsonDamageRolls::Seq(v) => v.into_iter().filter_map(|dmg| CreatureDamage::try_from(dmg).ok()).collect(),
                        },
                        traits: data.traits.into(),
                    };
                    if !attack.damage.is_empty() {
                        attacks.push(attack);
                    }
                }
                CreatureItemType::Skill => {
                    let skill = Skill::iter().find(|s| s.as_ref() == item.name).unwrap_or(Skill::Lore(item.name));
                    let data: JsonCreatureItemData = serde_json::from_value(item.data).expect("Could not deserialize skill data");
                    skills.push((skill, data.bonus.expect("this should have a bonus").value.into()));
                }
                // The assumption here is that relevant spellcasting entries will be visited before
                // their spells. If that doesn’t hold, change it here.
                CreatureItemType::SpellcastingEntry => {
                    let data: JsonSpellcastingEntry = serde_json::from_value(item.data).expect("Could not deserialize spellcasting entry");
                    let mut slots = BTreeMap::new();
                    slots.insert(0, data.slots.slot0.max.into());
                    slots.insert(1, data.slots.slot1.max.into());
                    slots.insert(2, data.slots.slot2.max.into());
                    slots.insert(3, data.slots.slot3.max.into());
                    slots.insert(4, data.slots.slot4.max.into());
                    slots.insert(5, data.slots.slot5.max.into());
                    slots.insert(6, data.slots.slot6.max.into());
                    slots.insert(7, data.slots.slot7.max.into());
                    slots.insert(8, data.slots.slot8.max.into());
                    slots.insert(9, data.slots.slot9.max.into());
                    slots.insert(10, data.slots.slot10.max.into());
                    spellcasting.push(SpellCasting {
                        name: item.name,
                        dc: data.spelldc.value.unwrap_or(-10) + 10,
                        spells: Vec::new(),
                        id: item._id,
                        slots,
                        casting_type: data.casting_type.value,
                    });
                }
                CreatureItemType::Spell => {
                    let data: JsonSpellData = serde_json::from_value(item.data).expect("Could not deserialize spell data");
                    let location: String = data.location.value.clone().into();
                    let casting = spellcasting
                        .iter_mut()
                        .find(|s| s.id == location)
                        .expect("Could not find spellcasting entry");
                    let spell = Spell::from(JsonSpell {
                        name: item.name.trim_end_matches(" - Cantrips").to_string(),
                        data,
                    });
                    casting.spells.push(spell);
                }
                CreatureItemType::Action => {
                    let ja = JsonAction {
                        name: item.name,
                        data: serde_json::from_value(item.data).expect("Could not deserialize action data"),
                    };
                    actions.push(ja.into());
                }
                _ => (),
            }
        }
        for c in spellcasting.iter_mut() {
            c.spells.sort();
        }

        Creature {
            name: jc.name,
            ability_scores: jc.data.abilities.into(),
            ac: jc.data.attributes.ac.value.into(),
            ac_details: remove_parentheses(jc.data.attributes.ac.details),
            hp: jc.data.attributes.hp.value.into(),
            hp_details: remove_parentheses(jc.data.attributes.hp.details),
            perception: jc.data.attributes.perception.value,
            senses: senses_as_string(jc.data.traits.senses),
            speeds: jc.data.attributes.speed.into(),
            flavor_text: jc.data.details.public_notes,
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
            spellcasting,
            actions,
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

fn senses_as_string(s: StringWrapperOrList) -> String {
    match s {
        StringWrapperOrList::Wrapper(w) => w.value,
        StringWrapperOrList::List(l) => l.join(", "),
        StringWrapperOrList::WrapperList(l) => l.into_iter().map(|w| w.value).filter(|s| !s.is_empty()).join(", "),
    }
    .trim_start_matches(", ")
    .trim_start_matches("; ")
    .to_string()
}

fn remove_parentheses(s: String) -> Option<String> {
    Some(s.trim_start_matches('(').trim_end_matches(')').to_string()).filter(|d| !d.is_empty())
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
#[serde(untagged)]
enum JsonNpc {
    JsonCreature(JsonCreature),
    JsonHazard(JsonHazard),
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreature {
    data: JsonCreatureData,
    name: String,
    items: Vec<JsonCreatureItem>,
    #[serde(rename = "type")]
    t: CreatureType,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
struct JsonHazard {
    data: JsonHazardData,
    name: String,
    items: Vec<JsonCreatureItem>,
    #[serde(rename = "type")]
    t: HazardType,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
struct JsonHazardData {
    details: JsonHazardDetails,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
struct JsonHazardDetails {
    level: ValueWrapper<i32>,
}

// Both markers for serde
#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum CreatureType {
    Npc,
}
#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum HazardType {
    Hazard,
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
    ac: ValueWithDetails,
    all_saves: Option<ValueWrapper<Option<String>>>,
    hp: ValueWithDetails,
    perception: ValueWrapper<i32>,
    speed: JsonCreatureSpeeds,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ValueWithDetails {
    value: StringOrNum,
    #[serde(default)]
    details: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatureSpeeds {
    pub value: String,
    pub other_speeds: Vec<OtherCreatureSpeed>,
}

impl From<JsonCreatureSpeeds> for CreatureSpeeds {
    fn from(j: JsonCreatureSpeeds) -> Self {
        CreatureSpeeds {
            value: ensure_trailing_unit(&String::from(j.value)),
            other_speeds: j
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

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JsonCreatureSpeeds {
    pub value: StringOrNum,
    pub other_speeds: Vec<OtherCreatureSpeed>,
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
    public_notes: Option<String>,
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
    WrapperList(Vec<ValueWrapper<String>>),
}

#[derive(Deserialize, PartialEq, Debug)]
struct JsonResistanceOrWeakness {
    #[serde(rename = "type")]
    damage_type: String,
    value: Option<StringOrNum>,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Eq)]
struct JsonCreatureItem {
    data: Value,
    #[serde(rename = "type")]
    item_type: CreatureItemType,
    name: String,
    _id: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JsonCreatureItemData {
    #[serde(alias = "mod")]
    bonus: Option<ValueWrapper<StringOrNum>>,
    traits: JsonTraits,
    #[serde(default)]
    damage_rolls: JsonDamageRolls,
    #[serde(default)]
    attack_effects: ValueWrapper<Vec<String>>,
    // range?
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum JsonDamageRolls {
    Map(BTreeMap<String, JsonCreatureDamage>),
    Seq(Vec<JsonCreatureDamage>),
}

impl Default for JsonDamageRolls {
    fn default() -> Self {
        JsonDamageRolls::Seq(vec![])
    }
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
pub(crate) struct JsonSpellcastingEntry {
    spelldc: ValueWrapper<Option<i32>>,
    slots: JsonSpellSlots,
    #[serde(rename = "prepared")]
    casting_type: ValueWrapper<SpellCastingType>,
}

// These often seem to be empty. Where are the slots stored then?
#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct JsonSpellSlots {
    slot0: JsonSpellSlot,
    slot1: JsonSpellSlot,
    slot2: JsonSpellSlot,
    slot3: JsonSpellSlot,
    slot4: JsonSpellSlot,
    slot5: JsonSpellSlot,
    slot6: JsonSpellSlot,
    slot7: JsonSpellSlot,
    slot8: JsonSpellSlot,
    slot9: JsonSpellSlot,
    slot10: JsonSpellSlot,
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct JsonSpellSlot {
    max: StringOrNum,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, AsRefStr, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum SpellCastingType {
    Prepared,
    Spontaneous,
    #[serde(alias = "Innate")]
    Innate,
    Ritual,
    Focus,
}

impl SpellCastingType {
    pub fn has_slots(&self) -> bool {
        self == &SpellCastingType::Spontaneous
    }

    pub fn has_dc(&self) -> bool {
        self != &SpellCastingType::Ritual
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(rename_all = "camelCase")]
enum CreatureItemType {
    #[serde(alias = "melee")]
    Weapon,
    // Includes passives
    Action,
    #[serde(rename = "lore")]
    Skill,

    SpellcastingEntry,
    Spell,

    Equipment,
    Consumable,
    Condition,
    Armor,
    Effect,
    Treasure,
    Feat,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{action_type::ActionType, damage::DamageType},
        tests::read_test_file,
    };

    #[test]
    fn test_deserialize_budget_dahak() {
        let dargon: Npc =
            serde_json::from_str(&read_test_file("pathfinder-bestiary.db/ancient-red-dragon.json")).expect("deserialization failed");
        let dargon = match dargon {
            Npc::Creature(c) => c,
            _ => panic!("Should have been a creature"),
        };
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
        match dargon.spellcasting.as_slice() {
            [spellcasting] => {
                assert_eq!(spellcasting.spells.len(), 4);
                assert_eq!(
                    spellcasting.spells.iter().map(|s| &s.name).collect_vec(),
                    ["Detect Magic", "Read Aura", "Suggestion (At Will)", "Wall of Fire (At Will)"]
                );
            }
            _ => assert!(false),
        }
        let expected_actions = vec![
            Action {
                name: "Smoke Vision".to_string(),
                description: "<p>Smoke doesn't impair a red dragon's vision; it ignores the concealed condition from smoke.</p>".to_string(),
                action_type: ActionType::Passive,
                number_of_actions: None,
                traits: Traits {
                    misc: vec![],
                    rarity: Rarity::Common,
                    alignment: None,
                    size: None
                }
            },
            Action {
                name: "Darkvision".to_string(),
                description: "<p>@Localize[PF2E.NPC.Abilities.Glossary.Darkvision]</p>".to_string(),
                action_type: ActionType::Passive,
                number_of_actions: None,
                traits: Traits {
                    misc: vec![],
                    rarity: Rarity::Common,
                    alignment: None,
                    size: None
                }
            },
            Action {
                name: "Scent (Imprecise) 60 feet".to_string(),
                description: "<p>@Localize[PF2E.NPC.Abilities.Glossary.Scent]</p>".to_string(),
                action_type: ActionType::Passive,
                number_of_actions: None,
                traits: Traits {
                    misc: vec![],
                    rarity: Rarity::Common,
                    alignment: None,
                    size: None
                }
            },
            Action {
                name: "At-Will Spells".to_string(),
                description: "<p>@Localize[PF2E.NPC.Abilities.Glossary.AtWillSpells]</p>".to_string(),
                action_type: ActionType::Passive,
                number_of_actions: None,
                traits: Traits {
                    misc: vec![],
                    rarity: Rarity::Common,
                    alignment: None,
                    size: None
                }
            },
            Action {
                name: "Dragon Heat".to_string(),
                description: "<p>10 feet, 4d6 fire damage</p>".to_string(),
                action_type: ActionType::Passive,
                number_of_actions: None,
                traits: Traits {
                    misc: vec!["arcane".to_string(), "aura".to_string(), "evocation".to_string(), "fire".to_string()],
                    rarity: Rarity::Common,
                    alignment: None,
                    size: None
                }
            },
            Action {
                name: "Frightful Presence".to_string(),
                description: "<p>90 feet, <a href=\"/creature_abilities/aura\">Aura</a> DC 40 Will</p>\n<p>@Localize[PF2E.NPC.Abilities.Glossary.FrightfulPresence]</p>".to_string(),
                action_type: ActionType::Passive,
                number_of_actions: None,
                traits: Traits { misc: vec!["aura".to_string(), "emotion".to_string(), "fear".to_string(), "mental".to_string()], rarity: Rarity::Common, alignment: None, size: None }
            },
            Action {
                name: "Attack of Opportunity".to_string(),
                description: "<p>Jaws only</p>\n<hr />\n<p>@Localize[PF2E.NPC.Abilities.Glossary.AttackOfOpportunity]</p>".to_string(),
                action_type: ActionType::Reaction,
                number_of_actions: None,
                traits: Traits { misc: vec![], rarity: Rarity::Common, alignment: None, size: None }
            },

            Action {
                name: "Redirect Fire".to_string(),
                description: "<p><strong>Trigger</strong> A creature within 100 feet casts a fire spell, or a fire spell otherwise comes into effect from a source within 100 feet.</p>\n<p><strong>Effect</strong> The dragon makes all the choices to determine the targets, destination, or other effects of the spell, as though it were the caster.</p>".to_string(),
                action_type: ActionType::Reaction,
                number_of_actions: None,
                traits: Traits { misc: vec!["abjuration".to_string(), "arcane".to_string()], rarity: Rarity::Common, alignment: None, size: None }
            },
            Action {
                name: "+1 Status to All Saves vs Magic".to_string(),
                description: String::new(),
                action_type: ActionType::Passive,
                number_of_actions: None,
                traits: Traits {
                    misc: vec![],
                    rarity: Rarity::Common,
                    alignment: None,
                    size: None
                }
            },
            Action {
                name: "Breath Weapon".to_string(),
                description: "<p>The dragon breathes a blast of flame that deals 20d6 fire damage in a 60-foot cone (DC 42 basic Reflex save). It can't use Breath Weapon again for 1d4 rounds.</p>".to_string(),
                action_type: ActionType::Action,
                number_of_actions: Some(2),
                traits: Traits { misc: vec!["arcane".to_string(), "evocation".to_string(), "fire".to_string()], rarity: Rarity::Common, alignment: None, size: None }
            },
            Action {
                name: "Draconic Frenzy".to_string(),
                description: "<p>The dragon makes two claw Strikes and one wing Strike in any order.</p>".to_string(),
                action_type: ActionType::Action,
                number_of_actions: Some(2),
                traits: Traits { misc: vec![], rarity: Rarity::Common, alignment: None, size: None }
            },
            Action {
                name: "Draconic Momentum".to_string(),
                description: "<p>The dragon recharges its Breath Weapon whenever it scores a critical hit with a Strike.</p>".to_string(),
                action_type: ActionType::Passive,
                number_of_actions: None,
                traits: Traits { misc: vec![], rarity: Rarity::Common, alignment: None, size: None }
            },
            Action {
                name: "Manipulate Flames".to_string(),
                description: "<p>The red dragon attempts to take control of a magical fire or a fire spell within 100 feet. If it succeeds at a counteract check (counteract level 10, counteract modifier +32), the original caster loses control of the spell or magic fire, control is transferred to the dragon, and the dragon counts as having Sustained the Spell with this action (if applicable). The dragon can choose to end the spell instead of taking control, if it chooses.</p>".to_string(),
                action_type: ActionType::Action,
                number_of_actions: Some(1),
                traits: Traits { misc: vec!["arcane".to_string(), "concentrate".to_string(), "transmutation".to_string()], rarity: Rarity::Common, alignment: None, size: None }
            }
        ];
        assert_eq!(dargon.actions, expected_actions);
    }

    #[test]
    fn prepared_caster_test() {
        let lich: Npc = serde_json::from_str(&read_test_file("pathfinder-bestiary.db/lich.json")).expect("deserialization failed");
        let lich = match lich {
            Npc::Creature(c) => c,
            _ => panic!("Should have been a creature"),
        };
        let mm = lich.spellcasting[0]
            .spells
            .iter()
            .find(|s| s.name == "Magic Missile")
            .expect("MM not found");
        assert_eq!(mm.level(), 3);
        assert_eq!(mm.level, 1);
        assert_eq!(mm.prepared_level, Some(3));
    }
}
