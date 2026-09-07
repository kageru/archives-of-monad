use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::sync::LazyLock;
use strum::IntoEnumIterator;

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

#[derive(Serialize, Debug, PartialEq, Clone, Eq)]
pub struct EquipmentDamageWithSplash<'a>(pub &'a EquipmentDamage, pub i32);

impl fmt::Display for EquipmentDamageWithSplash<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<b>Damage</b> {}", self.0)?;
        if self.1 != 0 {
            write!(f, " (plus {} splash damage)", self.1)?;
        }
        Ok(())
    }
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

static DAMAGE_TYPES_LOWERCASED: LazyLock<Vec<(DamageType, String)>> =
    LazyLock::new(|| DamageType::iter().map(|dt| (dt, dt.as_ref().to_lowercase())).collect());

impl DamageType {
    pub fn from_str_lower(name: &str) -> Option<DamageType> {
        DAMAGE_TYPES_LOWERCASED.iter().find(|(_, s)| s == name).map(|(v, _)| *v)
    }
}
