use super::{
    damage::{Damage, DamageScaling, DamageType},
    traits::{JsonTraits, Traits},
    HasName, I32Wrapper, ValueWrapper,
};
use crate::replace_references;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Display};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(from = "JsonSpell")]
pub struct Spell {
    pub name: String,
    pub area: Area,
    pub basic_save: bool,
    pub area_string: Option<String>, // not happy with this
    pub components: SpellComponents,
    pub cost: String,
    pub category: SpellCategory,
    pub damage: Damage,
    pub damage_type: DamageType,
    pub description: String,
    pub duration: String,
    pub level: i32,
    pub range: String,
    pub save: Option<Save>,
    pub scaling: DamageScaling,
    pub school: SpellSchool,
    pub secondary_casters: String,
    pub secondary_check: String,
    pub spell_type: SpellType,
    pub sustained: bool,
    pub target: String,
    pub time: String,
    pub primary_check: String,
    pub traditions: Vec<SpellTradition>,
    pub traits: Traits,
}

impl Spell {
    pub fn is_cantrip(&self) -> bool {
        self.traits.value.iter().any(|t| t == "cantrip")
    }
}

impl PartialOrd for Spell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Spell {
    fn cmp(&self, other: &Self) -> Ordering {
        fn level_for_sorting(s: &Spell) -> i32 {
            if s.is_cantrip() {
                0
            } else {
                s.level
            }
        }
        match level_for_sorting(self).cmp(&level_for_sorting(other)) {
            Ordering::Equal => self.name.cmp(&other.name),
            o => o,
        }
    }
}

impl HasName for Spell {
    fn name(&self) -> &str {
        &self.name
    }
}

impl From<JsonSpell> for Spell {
    fn from(js: JsonSpell) -> Self {
        let basic_save = js.data.save.basic == "basic";
        let save = match js.data.save.value.as_str() {
            "reflex" => Some(Save::Reflex),
            "fortitude" => Some(Save::Fortitude),
            "will" => Some(Save::Will),
            _ => None,
        };

        Spell {
            name: js.name.clone(),
            basic_save,
            save,
            area: match (js.data.area.area_type.as_str(), js.data.area.value.map(|s| s.0)) {
                ("cone", Some(ft)) => Area::Cone(ft),
                ("burst", Some(ft)) => Area::Burst(ft),
                ("emanation", Some(ft)) => Area::Emanation(ft),
                ("radius", Some(ft)) => Area::Radius(ft),
                ("line", Some(ft)) => Area::Line(ft),
                ("", None | Some(0)) => Area::None,
                (t, r) => unreachable!("Invalid spell area parameters: ({}, {:?})", t, r),
            },
            area_string: js.data.areasize.map(|v| v.value).filter(|v| !v.is_empty()),
            components: js.data.components,
            cost: js.data.cost.value,
            category: js.data.category.value,
            damage: js.data.damage,
            damage_type: js.data.damage_type.value,
            description: replace_references(&js.data.description.value),
            duration: js.data.duration.value,
            level: js.data.level.value,
            range: js.data.range.value,
            scaling: js.data.scaling,
            school: js.data.school.value,
            secondary_casters: js.data.secondarycasters.value,
            secondary_check: js.data.secondarycheck.value,
            primary_check: js.data.primarycheck.value,
            spell_type: js.data.spell_type.value,
            sustained: js.data.sustained.value,
            target: js.data.target.value,
            time: js.data.time.value,
            traditions: js.data.traditions.value,
            traits: js.data.traits.into(),
        }
    }
}

#[derive(Serialize, Debug, PartialEq, Clone, Copy, Eq)]
pub enum Area {
    Cone(i32),
    Burst(i32),
    Emanation(i32),
    Radius(i32),
    Line(i32),
    None,
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Area::Cone(v) => write!(f, "{}-foot cone", v),
            Area::Burst(v) => write!(f, "{}-foot burst", v),
            Area::Emanation(v) => write!(f, "{}-foot emanation", v),
            Area::Radius(v) => write!(f, "{}-foot radius", v),
            Area::Line(v) => write!(f, "{}-foot line", v),
            _ => write!(f, ""),
        }
    }
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
    areasize: Option<ValueWrapper<String>>,
    components: SpellComponents,
    cost: ValueWrapper<String>,
    category: ValueWrapper<SpellCategory>,
    damage: Damage,
    damage_type: ValueWrapper<DamageType>,
    description: ValueWrapper<String>,
    duration: ValueWrapper<String>,
    level: ValueWrapper<i32>,
    range: ValueWrapper<String>,
    save: JsonSave,
    scaling: DamageScaling,
    school: ValueWrapper<SpellSchool>,
    #[serde(default)]
    secondarycasters: ValueWrapper<String>,
    #[serde(default)]
    secondarycheck: ValueWrapper<String>,
    spell_type: ValueWrapper<SpellType>,
    sustained: ValueWrapper<bool>,
    target: ValueWrapper<String>,
    time: ValueWrapper<String>,
    #[serde(default)]
    primarycheck: ValueWrapper<String>,
    traditions: ValueWrapper<Vec<SpellTradition>>,
    traits: JsonTraits,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JsonSpellArea {
    #[serde(default)]
    area_type: String,
    value: Option<I32Wrapper>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct JsonSave {
    basic: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Display, Clone, Copy, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Save {
    Reflex,
    Fortitude,
    Will,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Eq)]
pub struct SpellComponents {
    somatic: bool,
    verbal: bool,
    material: bool,
}

impl SpellComponents {
    pub fn as_str(&self) -> &'static str {
        match (self.material, self.somatic, self.verbal) {
            (true, true, true) => " (material, somatic, verbal)",
            (true, true, false) => " (material, somatic)",
            (true, false, true) => " (material, verbal)",
            (true, false, false) => " (material)",
            (false, true, true) => " (somatic, verbal)",
            (false, true, false) => " (somatic)",
            (false, false, true) => " (verbal)",
            (false, false, false) => "",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Display, Eq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SpellType {
    Attack,
    Heal,
    Save,
    Utility,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Display, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SpellCategory {
    Cantrip,
    Spell,
    Focus,
    Ritual,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Display, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SpellTradition {
    Arcane,
    Divine,
    Occult,
    Primal,
}

#[cfg(test)]
mod tests {
    use crate::tests::read_test_file;

    use super::*;

    #[test]
    fn test_heal_deserialization() {
        let raw = read_test_file("spells.db/heal.json");
        let heal: Spell = serde_json::from_str(&raw).expect("Deserialization failed");
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
        let resurrect: Spell = serde_json::from_str(&read_test_file("spells.db/resurrect.json")).expect("Deserialization failed");
        assert_eq!(resurrect.name.as_str(), "Resurrect");
        assert_eq!(resurrect.spell_type, SpellType::Heal);
        assert!(resurrect.traditions.is_empty());
        assert_eq!(resurrect.secondary_casters, "2");
        assert_eq!(resurrect.category, SpellCategory::Ritual);
        assert_eq!(resurrect.secondary_check, "Medicine, Society");
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
