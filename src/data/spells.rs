use super::{
    equipment::StringOrNum,
    traits::{JsonTraits, Traits},
    HasLevel, HasName, ValueWrapper, URL_REMOVE_CHARACTERS, URL_REPLACE_CHARACTERS,
};
use crate::text_cleanup;
use serde::{Deserialize, Serialize};
use std::fmt;

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
    // pub damage: SpellDamage,
    // pub damage_type: DamageType,
    pub description: String,
    pub duration: String,
    pub level: i32,
    pub range: String,
    pub save: Option<Save>,
    // pub scaling: DamageScaling,
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
    pub source: String,
}

impl Spell {
    pub fn is_cantrip(&self) -> bool {
        self.traits.misc.iter().any(|t| t == "cantrip")
    }
}

impl HasLevel for Spell {
    fn level(&self) -> i32 {
        if self.is_cantrip() {
            0
        } else {
            self.level
        }
    }
}

impl HasName for Spell {
    fn name(&self) -> &str {
        &self.name
    }

    fn url_name(&self) -> String {
        let lower = self.name().to_lowercase();
        let no_prefix = lower.trim_end_matches(" (at will)").trim_end_matches(" (constant)");
        let underscored = URL_REPLACE_CHARACTERS.replace_all(no_prefix, "_");
        URL_REMOVE_CHARACTERS.replace_all(underscored.as_ref(), "").to_string()
    }
}

impl From<JsonSpell> for Spell {
    fn from(js: JsonSpell) -> Self {
        let basic_save = js.system.save.basic == "basic";
        let save = match js.system.save.value.as_str() {
            "reflex" => Some(Save::Reflex),
            "fortitude" => Some(Save::Fortitude),
            "will" => Some(Save::Will),
            _ => None,
        };

        Spell {
            name: js.name.clone(),
            basic_save,
            save,
            area: match (js.system.area.area_type.as_str(), js.system.area.value.map(i32::from)) {
                ("cone", Some(ft)) => Area::Cone(ft),
                ("burst", Some(ft)) => Area::Burst(ft),
                ("emanation", Some(ft)) => Area::Emanation(ft),
                ("radius", Some(ft)) => Area::Radius(ft),
                ("line", Some(ft)) => Area::Line(ft),
                ("square", Some(ft)) => Area::Square(ft),
                ("cube", Some(ft)) => Area::Cube(ft),
                ("", _) => Area::None,
                (t, r) => unreachable!("Invalid spell area parameters: ({}, {:?})", t, r),
            },
            area_string: js.system.areasize.map(|v| v.value).filter(|v| !v.is_empty()),
            components: js.system.components,
            cost: js.system.cost.value,
            category: js.system.category.value,
            // damage: js.data.damage,
            // damage_type: js.data.damage_type.value,
            description: text_cleanup(&js.system.description.value),
            duration: js.system.duration.value,
            level: js.system.level.value,
            range: js.system.range.value,
            // scaling: js.data.scaling,
            school: js.system.school.value,
            secondary_casters: js.system.secondarycasters.value,
            secondary_check: js.system.secondarycheck.value,
            primary_check: js.system.primarycheck.value,
            spell_type: js.system.spell_type.value,
            sustained: js.system.sustained.value,
            target: js.system.target.value,
            time: js.system.time.value,
            traditions: js.system.traditions.value,
            traits: {
                let mut traits = Traits::from(js.system.traits);
                traits.misc.push(js.system.school.value.as_ref().to_owned());
                traits.misc.sort_unstable();
                traits
            },
            source: js.system.source.value,
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
    Square(i32),
    Cube(i32),
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
            Area::Square(v) => write!(f, "{}-foot square", v),
            Area::Cube(v) => write!(f, "{}-foot cube", v),
            Area::None => write!(f, ""),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub(super) struct JsonSpell {
    pub system: JsonSpellData,
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(super) struct JsonSpellData {
    area: JsonSpellArea,
    areasize: Option<ValueWrapper<String>>,
    components: SpellComponents,
    cost: ValueWrapper<String>,
    category: ValueWrapper<SpellCategory>,
    // damage: SpellDamage,
    // damage_type: ValueWrapper<DamageType>,
    description: ValueWrapper<String>,
    duration: ValueWrapper<String>,
    level: JsonSpellLevel,
    range: ValueWrapper<String>,
    save: JsonSave,
    // scaling: DamageScaling,
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
    source: ValueWrapper<String>,
    // empty for standalone spells, non-empty for spells in creatures
    #[serde(default)]
    pub location: ValueWrapper<StringOrNum>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(super) struct JsonSpellLevel {
    value: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JsonSpellArea {
    #[serde(default)]
    area_type: String,
    value: Option<StringOrNum>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct JsonSave {
    basic: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, IntoStaticStr, Clone, Copy, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Save {
    Reflex,
    Fortitude,
    Will,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Eq)]
pub struct SpellComponents {
    pub somatic: bool,
    pub verbal: bool,
    pub material: bool,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, AsRefStr, IntoStaticStr, Eq)]
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
    #[serde(alias = "")]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, IntoStaticStr, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SpellCategory {
    Cantrip,
    Spell,
    Focus,
    Ritual,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, AsRefStr, IntoStaticStr, Eq)]
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
        // assert_eq!(heal.damage_type, DamageType::Positive);
        // assert_eq!(heal.damage, SpellDamage::without_mod("1d8".into()));
        assert_eq!(
            heal.components,
            SpellComponents {
                material: false,
                somatic: false,
                verbal: false,
            }
        );
        assert_eq!(heal.source, "Pathfinder Core Rulebook".to_string());
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
        // assert_eq!(resurrect.damage_type, DamageType::None);
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

    #[test]
    fn url_name_at_will_test() {
        let resurrect: Spell = serde_json::from_str(&read_test_file("spells.db/resurrect.json")).expect("Deserialization failed");
        let at_will = Spell {
            name: "Darkness (At Will)".to_string(),
            ..resurrect
        };
        assert_eq!(at_will.url_name(), "darkness");
        assert_eq!(at_will.name(), "Darkness (At Will)");
        let constant = Spell {
            name: "True Seeing (Constant)".to_string(),
            ..at_will
        };
        assert_eq!(constant.url_name(), "true_seeing");
        assert_eq!(constant.name(), "True Seeing (Constant)");
    }
}
