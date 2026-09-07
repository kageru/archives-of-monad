use super::{
    HasLevel, HasName, Publication, ValueWrapper,
    actions::{Action, JsonAction},
    damage::{CreatureDamage, DamageType},
    ensure_trailing_unit,
    equipment::StringOrNum,
    size::Size,
    skills::Skill,
    spells::{JsonSpell, JsonSpellData, Spell, SpellCategory},
    traits::{JsonTraits, Rarity},
};
use crate::data::traits::Traits;
use convert_case::{Case, Casing};
use itertools::Itertools;
use serde::{Deserialize, Serialize, de::IgnoredAny};
use serde_json::Value;
use std::{collections::BTreeMap, convert::TryFrom};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(from = "JsonNpc")]
pub enum Npc {
    Creature(Box<Creature>),
    Hazard(Box<Hazard>),
    Vehicle(Box<Vehicle>),
    Character,
}

impl HasName for Npc {
    fn name(&self) -> &str {
        match self {
            Npc::Creature(c) => &c.name,
            Npc::Hazard(h) => &h.name,
            Npc::Vehicle(v) => &v.name,
            Npc::Character => "",
        }
    }
}

impl HasLevel for Npc {
    fn level(&self) -> i32 {
        match self {
            Npc::Creature(c) => c.level,
            Npc::Hazard(h) => h.level,
            Npc::Vehicle(v) => v.level,
            Npc::Character => 0,
        }
    }
}

impl From<JsonNpc> for Npc {
    fn from(j: JsonNpc) -> Self {
        match j {
            JsonNpc::Creature(c) => Npc::Creature(Box::new(c.into())),
            JsonNpc::Hazard(h) => Npc::Hazard(Box::new(h.into())),
            JsonNpc::Vehicle(v) => Npc::Vehicle(Box::new(v.into())),
            JsonNpc::Character(_) => Npc::Character,
            JsonNpc::Other(_) => Npc::Character,
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
    // Rituals a creature can perform aren't tied to a spellcasting entry the way prepared,
    // spontaneous, innate, or focus spells are, so they're tracked separately.
    pub rituals: Vec<Spell>,
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
            level: j.system.details.level.value,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(from = "JsonVehicle")]
pub struct Vehicle {
    name: String,
    level: i32,
}

impl From<JsonVehicle> for Vehicle {
    fn from(j: JsonVehicle) -> Self {
        Vehicle {
            name: j.name,
            level: j.system.details.level.value,
        }
    }
}

#[derive(Serialize, PartialEq, Debug, Clone, Eq)]
pub struct SpellCasting {
    pub name: String,
    pub dc: i32,
    pub attack_modifier: i32,
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
        let mut rituals = Vec::new();

        for item in jc.items {
            match item.item_type {
                CreatureItemType::Weapon => {
                    let name = &item.name;
                    let data: JsonCreatureItemData = serde_json::from_value(item.system)
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
                    let data: JsonCreatureLoreItemData = serde_json::from_value(item.system).expect("Could not deserialize skill data");
                    skills.push((skill, data.modifier.value.into()));
                }
                // The assumption here is that relevant spellcasting entries will be visited before
                // their spells. If that doesn’t hold, change it here.
                CreatureItemType::SpellcastingEntry => {
                    let data: JsonSpellcastingEntry =
                        serde_json::from_value(item.system).expect("Could not deserialize spellcasting entry");
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
                        dc: data.spelldc.dc.map(i32::from).unwrap_or(0),
                        attack_modifier: data.spelldc.attack_modifier.map(i32::from).unwrap_or(0),
                        spells: Vec::new(),
                        id: item._id,
                        slots,
                        casting_type: data.casting_type.value,
                    });
                }
                CreatureItemType::Spell => {
                    let data: JsonSpellData = serde_json::from_value(item.system).expect("Could not deserialize spell data");
                    let location: String = data.location.value.clone().map(String::from).unwrap_or_default();
                    let spell = Spell::from(JsonSpell {
                        name: item.name.trim_end_matches(" - Cantrips").to_string(),
                        system: data,
                    });
                    // Rituals aren't tied to a spellcasting entry (no shared spell slots, DC, or
                    // attack modifier), so a missing location there is expected, not an error.
                    match spellcasting.iter_mut().find(|s| s.id == location) {
                        Some(casting) => casting.spells.push(spell),
                        None if spell.category == SpellCategory::Ritual => rituals.push(spell),
                        None => eprintln!("Could not find spellcasting entry for spell {}", item.name),
                    }
                }
                CreatureItemType::Action => {
                    let ja = JsonAction {
                        name: item.name,
                        system: serde_json::from_value(item.system).expect("Could not deserialize action data"),
                    };
                    actions.push(ja.into());
                }
                _ => (),
            }
        }
        for c in spellcasting.iter_mut() {
            c.spells.sort();
        }
        rituals.sort();
        skills.extend(jc.system.skills.into_iter().map(|(name, val)| (skill_from_key(&name), val.base)));

        let senses = perception_senses_as_string(&jc.system.perception);
        let immunity_names: Vec<String> = jc.system.attributes.immunities.iter().map(|r| r.damage_type.clone()).collect();

        Creature {
            name: jc.name,
            ability_scores: jc.system.abilities.into(),
            ac: jc.system.attributes.ac.value.into(),
            ac_details: remove_parentheses(jc.system.attributes.ac.details),
            hp: jc.system.attributes.hp.value.into(),
            hp_details: remove_parentheses(jc.system.attributes.hp.details),
            perception: jc.system.perception.modifier,
            senses,
            speeds: jc.system.attributes.speed.into(),
            flavor_text: jc.system.details.public_notes,
            level: jc.system.details.level.value,
            source: jc.system.details.publication.title,
            saves: SavingThrows {
                reflex: jc.system.saves.reflex.value.into(),
                fortitude: jc.system.saves.fortitude.value.into(),
                will: jc.system.saves.will.value.into(),
                additional_save_modifier: jc.system.attributes.all_saves.and_then(|v| v.value),
            },
            traits: Traits {
                misc: titlecased(&jc.system.traits.value),
                rarity: jc.system.traits.rarity,
                size: Some(jc.system.traits.size.value),
                // The remaster removed creature alignment entirely; alignment-flavored traits like
                // "chaotic" or "evil" now just live in the regular trait list above.
                alignment: None,
            },
            resistances: jc.system.attributes.resistances.iter().map_into().collect(),
            weaknesses: jc.system.attributes.weaknesses.iter().map_into().collect(),
            immunities: lowercased(&immunity_names),
            languages: {
                let mut titlecased = titlecased(&jc.system.details.languages.value);
                if !jc.system.details.languages.custom.is_empty() {
                    titlecased.push(
                        jc.system
                            .details
                            .languages
                            .custom
                            // TODO: try if heck or a similar library handles these characters.
                            // convert_case apparently doesn’t and won’t, see https://github.com/rutrum/convert-case/issues/4
                            .replace('’', "'")
                            .from_case(Case::Lower)
                            .to_case(Case::Title),
                    );
                }
                titlecased
            },
            attacks,
            skills,
            spellcasting,
            rituals,
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

#[allow(clippy::large_enum_variant)]
#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum JsonNpc {
    Creature(JsonCreature),
    Hazard(JsonHazard),
    Vehicle(JsonVehicle),
    Character(JsonCharacter),
    // The bestiary compendia also ship unrelated documents (actions, effects, army units, ...)
    // alongside the actual creatures; we don't render those, so just ignore them.
    Other(IgnoredAny),
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreature {
    system: JsonCreatureData,
    name: String,
    items: Vec<JsonCreatureItem>,
    #[serde(rename = "type")]
    t: CreatureType,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
struct JsonHazard {
    system: JsonHazardData,
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

#[derive(Deserialize, Debug, PartialEq, Clone)]
struct JsonCharacter {
    #[serde(rename = "type")]
    t: CharacterType,
}
#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum CharacterType {
    Character,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
struct JsonVehicle {
    name: String,
    #[serde(rename = "type")]
    t: VehicleType,
    system: JsonVehicleData,
}
#[derive(Deserialize, Debug, PartialEq, Clone)]
struct JsonVehicleData {
    details: JsonVehicleDetails,
}
#[derive(Deserialize, Debug, PartialEq, Clone)]
struct JsonVehicleDetails {
    level: ValueWrapper<i32>,
}

// All just markers for serde
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
#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum VehicleType {
    Vehicle,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreatureData {
    abilities: JsonCreatureAbilities,
    attributes: JsonCreatureAttributes,
    details: JsonCreatureDetails,
    perception: JsonCreaturePerception,
    saves: JsonCreatureSaves,
    traits: JsonCreatureTraits, // different from usual traits
    #[serde(default)]
    skills: BTreeMap<String, JsonCreatureSkillValue>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreatureSkillValue {
    base: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JsonCreaturePerception {
    #[serde(rename = "mod")]
    modifier: i32,
    #[serde(default)]
    details: String,
    #[serde(default)]
    senses: Vec<JsonSense>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonSense {
    #[serde(rename = "type")]
    sense_type: String,
    #[serde(default)]
    acuity: Option<String>,
    #[serde(default)]
    range: Option<i32>,
}

fn skill_from_key(key: &str) -> Skill {
    serde_json::from_value(Value::String(key.to_string())).unwrap_or_else(|_| Skill::Lore(key.from_case(Case::Kebab).to_case(Case::Title)))
}

fn format_sense(s: &JsonSense) -> String {
    let name = s.sense_type.replace('-', " ");
    match (&s.acuity, s.range) {
        (Some(acuity), Some(range)) => format!("{name} ({acuity}) {range} feet"),
        (Some(acuity), None) => format!("{name} ({acuity})"),
        (None, Some(range)) => format!("{name} {range} feet"),
        (None, None) => name,
    }
}

fn perception_senses_as_string(p: &JsonCreaturePerception) -> String {
    p.senses
        .iter()
        .map(format_sense)
        .chain(std::iter::once(p.details.clone()).filter(|d| !d.is_empty()))
        .join(", ")
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
    speed: JsonCreatureSpeeds,
    #[serde(default)]
    resistances: Vec<JsonResistanceOrWeakness>,
    #[serde(default)]
    weaknesses: Vec<JsonResistanceOrWeakness>,
    #[serde(default)]
    immunities: Vec<JsonResistanceOrWeakness>,
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
            value: ensure_trailing_unit(&String::from(j.value.unwrap_or_default())),
            other_speeds: j
                .other_speeds
                .into_iter()
                .map(|speed| OtherCreatureSpeed {
                    speed_type: speed.speed_type,
                    value: ensure_trailing_unit(&String::from(speed.value)),
                })
                .collect(),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct JsonCreatureSpeeds {
    pub value: Option<StringOrNum>,
    pub other_speeds: Vec<JsonOtherCreatureSpeed>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct OtherCreatureSpeed {
    pub speed_type: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct JsonOtherCreatureSpeed {
    #[serde(rename = "type")]
    pub speed_type: String,
    pub value: StringOrNum,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct JsonCreatureDetails {
    public_notes: Option<String>,
    level: ValueWrapper<i32>,
    publication: Publication,
    languages: JsonLanguages,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct JsonCreatureSaves {
    fortitude: ValueWrapper<StringOrNum>,
    reflex: ValueWrapper<StringOrNum>,
    will: ValueWrapper<StringOrNum>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct JsonCreatureTraits {
    rarity: Rarity,
    size: ValueWrapper<Size>,
    #[serde(default)]
    value: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct JsonLanguages {
    #[serde(rename = "details")]
    custom: String,
    value: Vec<String>,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
struct JsonResistanceOrWeakness {
    #[serde(rename = "type")]
    damage_type: String,
    value: Option<StringOrNum>,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Eq)]
struct JsonCreatureItem {
    system: Value,
    #[serde(rename = "type")]
    item_type: CreatureItemType,
    name: String,
    _id: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
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

// Lore skill items ("type": "lore") have their own, much smaller shape.
#[derive(Deserialize, Debug, PartialEq, Eq)]
struct JsonCreatureLoreItemData {
    #[serde(rename = "mod")]
    modifier: ValueWrapper<StringOrNum>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct JsonSpellcastingEntry {
    spelldc: JsonSpellDC,
    slots: JsonSpellSlots,
    #[serde(rename = "prepared")]
    casting_type: ValueWrapper<SpellCastingType>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct JsonSpellDC {
    dc: Option<StringOrNum>,
    #[serde(rename = "value")]
    attack_modifier: Option<StringOrNum>,
}

// Empty for non-slot-based casting types (e.g. innate or focus casting).
#[derive(Deserialize, Debug, PartialEq, Eq, Default)]
pub(crate) struct JsonSpellSlots {
    #[serde(default)]
    slot0: JsonSpellSlot,
    #[serde(default)]
    slot1: JsonSpellSlot,
    #[serde(default)]
    slot2: JsonSpellSlot,
    #[serde(default)]
    slot3: JsonSpellSlot,
    #[serde(default)]
    slot4: JsonSpellSlot,
    #[serde(default)]
    slot5: JsonSpellSlot,
    #[serde(default)]
    slot6: JsonSpellSlot,
    #[serde(default)]
    slot7: JsonSpellSlot,
    #[serde(default)]
    slot8: JsonSpellSlot,
    #[serde(default)]
    slot9: JsonSpellSlot,
    #[serde(default)]
    slot10: JsonSpellSlot,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Default)]
pub(crate) struct JsonSpellSlot {
    #[serde(default)]
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
    // Spells cast by using up a consumable item (e.g. a held scroll), rather than a spell slot.
    Items,
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
    Backpack,
    Feat,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::read_test_file;

    #[test]
    fn prepared_caster_test() {
        let lich: Npc = serde_json::from_str(&read_test_file("pathfinder-monster-core/lich.json")).expect("deserialization failed");
        let lich = match lich {
            Npc::Creature(c) => c,
            _ => panic!("Should have been a creature"),
        };
        let mm = lich.spellcasting[0]
            .spells
            .iter()
            .find(|s| s.name == "Force Barrage")
            .expect("Force Barrage not found");
        assert_eq!(mm.level(), 1);
        assert_eq!(mm.level, 1); // TODO: find real level after the changes; this should be 6 because it’s prepared at 6
    }
}
