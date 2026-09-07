use super::{
    HasLevel, HasName, Publication, URL_REMOVE_CHARACTERS, URL_REPLACE_CHARACTERS, ValueWrapper,
    equipment::StringOrNum,
    traits::{JsonTraits, Traits},
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
    pub cost: String,
    pub category: SpellCategory,
    pub description: String,
    pub duration: String,
    pub level: i32,
    pub range: String,
    pub save: Option<Save>,
    pub secondary_casters: String,
    pub secondary_check: String,
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
        if self.is_cantrip() { 0 } else { self.level }
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
        let json_save = js.system.defense.as_ref().and_then(|d| d.save.as_ref());
        let basic_save = json_save.map(|s| s.basic).unwrap_or(false);
        let save = json_save.and_then(|s| match s.statistic.as_str() {
            "reflex" => Some(Save::Reflex),
            "fortitude" => Some(Save::Fortitude),
            "will" => Some(Save::Will),
            _ => None,
        });
        let is_ritual = js.system.ritual.is_some();
        let is_focus = js.system.traits.base.value.iter().any(|t| t == "focus");

        Spell {
            name: js.name.clone(),
            basic_save,
            save,
            area: match js.system.area {
                Some(JsonSpellArea { area_type, value }) => match area_type.as_str() {
                    "cone" => Area::Cone(value),
                    "burst" => Area::Burst(value),
                    "emanation" => Area::Emanation(value),
                    "radius" => Area::Radius(value),
                    "line" => Area::Line(value),
                    "square" => Area::Square(value),
                    "cube" => Area::Cube(value),
                    "cylinder" => Area::Cylinder(value),
                    t => unreachable!("Invalid spell area type: {}", t),
                },
                None => Area::None,
            },
            area_string: None,
            cost: js.system.cost.value,
            category: if is_ritual {
                SpellCategory::Ritual
            } else if is_focus {
                SpellCategory::Focus
            } else {
                SpellCategory::Spell
            },
            description: text_cleanup(&js.system.description.value),
            duration: js.system.duration.value,
            level: js.system.level.value.into(),
            range: js.system.range.value,
            secondary_casters: js
                .system
                .ritual
                .as_ref()
                .and_then(|r| r.secondary.casters.clone())
                .map(String::from)
                .unwrap_or_default(),
            secondary_check: js.system.ritual.as_ref().map(|r| r.secondary.checks.clone()).unwrap_or_default(),
            primary_check: js.system.ritual.as_ref().map(|r| r.primary.check.clone()).unwrap_or_default(),
            sustained: js.system.duration.sustained,
            target: js.system.target.value,
            time: js.system.time.value,
            traditions: js.system.traits.traditions.clone(),
            traits: Traits::from(js.system.traits.base),
            source: js.system.publication.title,
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
    Cylinder(i32),
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
            Area::Cylinder(v) => write!(f, "{}-foot cylinder", v),
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
    area: Option<JsonSpellArea>,
    cost: ValueWrapper<String>,
    defense: Option<JsonSpellDefense>,
    description: ValueWrapper<String>,
    duration: JsonSpellDuration,
    level: JsonSpellLevel,
    range: ValueWrapper<String>,
    ritual: Option<JsonRitual>,
    target: ValueWrapper<String>,
    time: ValueWrapper<String>,
    traits: JsonSpellTraits,
    publication: Publication,
    // empty for standalone spells, non-empty for spells in creatures
    #[serde(default)]
    pub location: ValueWrapper<Option<StringOrNum>>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(super) struct JsonSpellLevel {
    value: StringOrNum,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JsonSpellArea {
    #[serde(rename = "type")]
    area_type: String,
    value: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonSpellDefense {
    save: Option<JsonSave>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct JsonSave {
    basic: bool,
    statistic: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonSpellDuration {
    #[serde(default)]
    sustained: bool,
    value: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonRitual {
    primary: JsonRitualCheck,
    secondary: JsonRitualSecondary,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonRitualCheck {
    check: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonRitualSecondary {
    casters: Option<StringOrNum>,
    checks: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonSpellTraits {
    #[serde(flatten)]
    base: JsonTraits,
    #[serde(default)]
    traditions: Vec<SpellTradition>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, IntoStaticStr, Clone, Copy, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Save {
    Reflex,
    Fortitude,
    Will,
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
        let raw = read_test_file("spells/spells/rank-1/heal.json");
        let heal: Spell = serde_json::from_str(&raw).expect("Deserialization failed");
        assert_eq!(heal.name.as_str(), "Heal");
        assert_eq!(heal.category, SpellCategory::Spell);
        assert_eq!(heal.traditions, vec![SpellTradition::Divine, SpellTradition::Primal]);
        assert_eq!(heal.source, "Pathfinder Player Core".to_string());
    }

    #[test]
    fn test_resurrect_deserialization() {
        let resurrect: Spell = serde_json::from_str(&read_test_file("spells/rituals/resurrect.json")).expect("Deserialization failed");
        assert_eq!(resurrect.name.as_str(), "Resurrect");
        assert!(resurrect.traditions.is_empty());
        assert_eq!(resurrect.secondary_casters, "2");
        assert_eq!(resurrect.category, SpellCategory::Ritual);
        assert_eq!(resurrect.secondary_check, "Medicine, Society");
        assert_eq!(resurrect.time, "1 day");
        assert_eq!(resurrect.cost, "gemstones worth a total value of 75 gp × the target's level");
    }

    #[test]
    fn url_name_at_will_test() {
        let resurrect: Spell = serde_json::from_str(&read_test_file("spells/rituals/resurrect.json")).expect("Deserialization failed");
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
