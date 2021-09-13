use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
pub struct SpellDamage {
    #[serde(rename = "value")]
    pub formula: String,
    #[serde(rename = "applyMod")]
    pub apply_mod: bool,
}

impl SpellDamage {
    #[allow(unused)]
    pub fn without_mod(formula: String) -> Self {
        SpellDamage { formula, apply_mod: false }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreatureDamage {
    pub damage: String,
    pub damage_type: DamageType,
}

// Equipment and spell damage is structured differently.
// We should at some point parse one into the other.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
pub struct EquipmentDamage {
    pub damage_type: DamageType,
    pub die: Die,
    pub number_of_dice: i32,
}

impl fmt::Display for EquipmentDamage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{} {}", self.number_of_dice, self.die, self.damage_type.as_ref(),)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Die {
    #[serde(alias = "")]
    NoDamage,
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

impl Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Die::NoDamage => "",
                Die::D4 => "d4",
                Die::D6 => "d6",
                Die::D8 => "d8",
                Die::D10 => "d10",
                Die::D12 => "d12",
                Die::D20 => "d20",
                Die::D100 => "d100",
            }
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
pub struct DamageScaling {
    pub formula: String,
    pub mode: DamageScalingMode,
}

#[derive(Serialize, PartialEq, Debug, Clone, Copy, Eq)]
pub enum DamageScalingMode {
    NoScaling,
    Every(i32),
}

impl<'de> Deserialize<'de> for DamageScalingMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "level1" => Ok(DamageScalingMode::Every(1)),
            "level2" => Ok(DamageScalingMode::Every(2)),
            "level4" => Ok(DamageScalingMode::Every(4)),
            _ => Ok(DamageScalingMode::NoScaling),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Eq, AsRefStr, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum DamageType {
    Acid,
    Bleed,
    Bludgeoning,
    Chaotic,
    Cold,
    Electricity,
    Evil,
    Fire,
    Force,
    Good,
    Healing,
    Lawful,
    Mental,
    Negative,
    Piercing,
    Precision, // technically not a damage type itself, but it appears in the data
    Poison,
    Positive,
    Slashing,
    Sonic,
    TempHp,
    #[serde(rename = "")]
    None,
}
