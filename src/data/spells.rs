use super::{
    damage::{Damage, DamageScaling, DamageType},
    traits::{JsonTraits, Traits},
    ValueWrapper,
};
use crate::impl_deser;
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
struct Spell {
    name: String,
    area: Area,
    area_string: String, // not happy with this
    components: SpellComponents,
    cost: String,
    damage: Damage,
    damage_type: DamageType,
    description: String,
    duration: String,
    level: i32,
    range: String,
    scaling: DamageScaling,
    school: SpellSchool,
    secondary_casters: i32,
    secondary_check: Option<String>,
    spell_type: SpellType,
    sustained: bool,
    target: String,
    time: String,
    traditions: Vec<SpellTradition>,
    traits: Traits,
}

impl From<JsonSpell> for Spell {
    fn from(js: JsonSpell) -> Self {
        Spell {
            name: js.name,
            area: match (js.data.area.area_type.as_str(), js.data.area.value) {
                ("cone", Some(ft)) => Area::Cone(ft),
                ("burst", Some(ft)) => Area::Burst(ft),
                ("emanation", Some(ft)) => Area::Emanation(ft),
                ("", None) => Area::None,
                (t, r) => unreachable!("Invalid spell area parameters: ({}, {:?})", t, r),
            },
            area_string: js.data.areasize.value,
            components: js.data.components,
            cost: js.data.cost.value,
            damage: js.data.damage,
            damage_type: js.data.damage_type.value,
            description: js.data.description.value,
            duration: js.data.duration.value,
            level: js.data.level.value,
            range: js.data.range.value,
            scaling: js.data.scaling,
            school: js.data.school.value,
            secondary_casters: js.data.secondarycasters.value.parse().unwrap_or(0),
            secondary_check: Some(js.data.secondarycheck.value).filter(|s| !s.is_empty()),
            spell_type: js.data.spell_type.value,
            sustained: js.data.sustained.value,
            target: js.data.target.value,
            time: js.data.time.value,
            traditions: js.data.traditions.value,
            traits: js.data.traits.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Area {
    Cone(i32),
    Burst(i32),
    Emanation(i32),
    None,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonSpell {
    data: JsonSpellData,
    name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonSpellData {
    area: JsonSpellArea,
    areasize: ValueWrapper<String>,
    components: SpellComponents,
    cost: ValueWrapper<String>,
    damage: Damage,
    #[serde(rename = "damageType")]
    damage_type: ValueWrapper<DamageType>,
    description: ValueWrapper<String>,
    duration: ValueWrapper<String>,
    level: ValueWrapper<i32>,
    range: ValueWrapper<String>,
    scaling: DamageScaling,
    school: ValueWrapper<SpellSchool>,
    secondarycasters: ValueWrapper<String>,
    secondarycheck: ValueWrapper<String>,
    #[serde(rename = "spellType")]
    spell_type: ValueWrapper<SpellType>,
    sustained: ValueWrapper<bool>,
    target: ValueWrapper<String>,
    time: ValueWrapper<String>,
    traditions: ValueWrapper<Vec<SpellTradition>>,
    traits: JsonTraits,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonSpellArea {
    #[serde(rename = "areaType")]
    area_type: String,
    value: Option<i32>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct SpellComponents {
    somatic: bool,
    verbal: bool,
    material: bool,
}

#[derive(Debug, PartialEq)]
enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

impl_deser! {
    SpellSchool :
    "abjuration" => SpellSchool::Abjuration,
    "conjuration" => SpellSchool::Conjuration,
    "divination" => SpellSchool::Divination,
    "enchantment" => SpellSchool::Enchantment,
    "evocation" => SpellSchool::Evocation,
    "illusion" => SpellSchool::Illusion,
    "necromancy" => SpellSchool::Necromancy,
    "transmutation" => SpellSchool::Transmutation,
    expects: "(Abjuration|Conjuration|Divination|Enchantment|Evocation|Illusion|Necromancy|Transmutation)"
}

#[derive(Debug, PartialEq)]
enum SpellType {
    Attack,
    Heal,
    Save,
    Utility,
}

impl_deser! {
    SpellType :
    "attack" => SpellType::Attack,
    "heal" => SpellType::Heal,
    "save" => SpellType::Save,
    "utility" => SpellType::Utility,
    expects: "(attack|heal|save|utility)"
}

#[derive(Debug, PartialEq)]
enum SpellTradition {
    Arcane,
    Divine,
    Occult,
    Primal,
}

impl_deser! {
    SpellTradition :
    "arcane" => SpellTradition::Arcane,
    "divine" => SpellTradition::Divine,
    "occult" => SpellTradition::Occult,
    "primal" => SpellTradition::Primal,
    expects: "(arcane|divine|occult|primal)"
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_heal_deserialization() {
        let f = std::fs::File::open("tests/data/spells/heal.json").expect("File missing");
        let reader = BufReader::new(f);
        let heal = Spell::from(serde_json::from_reader::<_, JsonSpell>(reader).expect("Deserialization failed"));
        assert_eq!(heal.name.as_str(), "Heal");
        assert_eq!(heal.spell_type, SpellType::Heal);
        assert_eq!(heal.school, SpellSchool::Necromancy);
        assert_eq!(heal.traditions, vec![SpellTradition::Divine, SpellTradition::Primal]);
        assert_eq!(heal.damage_type, DamageType::Positive);
        assert_eq!(heal.damage, Damage::without_mod("1d8".into()));
        assert_eq!(
            heal.components,
            SpellComponents {
                material: false,
                somatic: false,
                verbal: false,
            }
        );
    }

    #[test]
    fn test_resurrect_deserialization() {
        let f = std::fs::File::open("tests/data/spells/resurrect.json").expect("File missing");
        let reader = BufReader::new(f);
        let resurrect = Spell::from(serde_json::from_reader::<_, JsonSpell>(reader).expect("Deserialization failed"));
        assert_eq!(resurrect.name.as_str(), "Resurrect");
        assert_eq!(resurrect.spell_type, SpellType::Heal);
        assert!(resurrect.traditions.is_empty());
        assert_eq!(resurrect.secondary_casters, 2);
        assert_eq!(resurrect.secondary_check, Some("Medicine, Society".into()));
        assert_eq!(resurrect.time, "1 day");
        assert_eq!(resurrect.cost, "diamonds worth a total value of 75 gp × the target's level");
    }

    #[test]
    fn test_spelltype() {
        assert_eq!(
            serde_json::from_str::<ValueWrapper<SpellType>>(r#"{"value": "attack"}"#)
                .unwrap()
                .value,
            SpellType::Attack
        );
    }
}
