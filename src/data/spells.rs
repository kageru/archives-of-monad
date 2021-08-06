use super::{
    damage::{Damage, DamageScaling, DamageType},
    traits::{JsonTraits, Traits},
    HasName, ValueWrapper,
};
use core::fmt;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(from = "JsonSpell")]
pub struct Spell {
    pub name: String,
    pub area: Area,
    pub area_string: String, // not happy with this
    pub components: SpellComponents,
    pub cost: String,
    pub category: SpellCategory,
    pub damage: Damage,
    pub damage_type: DamageType,
    pub description: String,
    pub duration: String,
    pub level: i32,
    pub range: String,
    pub scaling: DamageScaling,
    pub school: SpellSchool,
    pub secondary_casters: i32,
    pub secondary_check: Option<String>,
    pub spell_type: SpellType,
    pub sustained: bool,
    pub target: String,
    pub time: String,
    pub traditions: Vec<SpellTradition>,
    pub traits: Traits,
}

impl HasName for Spell {
    fn name(&self) -> &str {
        &self.name
    }
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
            category: js.data.category.value,
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
pub enum Area {
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
#[serde(rename_all = "camelCase")]
struct JsonSpellData {
    area: JsonSpellArea,
    areasize: ValueWrapper<String>,
    components: SpellComponents,
    cost: ValueWrapper<String>,
    category: ValueWrapper<SpellCategory>,
    damage: Damage,
    damage_type: ValueWrapper<DamageType>,
    description: ValueWrapper<String>,
    duration: ValueWrapper<String>,
    level: ValueWrapper<i32>,
    range: ValueWrapper<String>,
    scaling: DamageScaling,
    school: ValueWrapper<SpellSchool>,
    secondarycasters: ValueWrapper<String>,
    secondarycheck: ValueWrapper<String>,
    spell_type: ValueWrapper<SpellType>,
    sustained: ValueWrapper<bool>,
    target: ValueWrapper<String>,
    time: ValueWrapper<String>,
    traditions: ValueWrapper<Vec<SpellTradition>>,
    traits: JsonTraits,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JsonSpellArea {
    area_type: String,
    value: Option<i32>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct SpellComponents {
    somatic: bool,
    verbal: bool,
    material: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SpellType {
    Attack,
    Heal,
    Save,
    Utility,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SpellCategory {
    Cantrip,
    Spell,
    Focus,
    Ritual,
}

impl fmt::Display for SpellCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SpellTradition {
    Arcane,
    Divine,
    Occult,
    Primal,
}

impl fmt::Display for SpellTradition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_heal_deserialization() {
        let f = std::fs::File::open("tests/data/spells/heal.json").expect("File missing");
        let reader = BufReader::new(f);
        let heal = serde_json::from_reader::<_, Spell>(reader).expect("Deserialization failed");
        assert_eq!(heal.name.as_str(), "Heal");
        assert_eq!(heal.spell_type, SpellType::Heal);
        assert_eq!(heal.category, SpellCategory::Spell);
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
        let resurrect = serde_json::from_reader::<_, Spell>(reader).expect("Deserialization failed");
        assert_eq!(resurrect.name.as_str(), "Resurrect");
        assert_eq!(resurrect.spell_type, SpellType::Heal);
        assert!(resurrect.traditions.is_empty());
        assert_eq!(resurrect.secondary_casters, 2);
        assert_eq!(resurrect.category, SpellCategory::Ritual);
        assert_eq!(resurrect.secondary_check, Some("Medicine, Society".into()));
        assert_eq!(resurrect.time, "1 day");
        assert_eq!(resurrect.damage_type, DamageType::None);
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
