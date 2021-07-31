use serde::{Deserialize, Deserializer};
use crate::impl_deser;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Damage {
    #[serde(rename = "value")]
    pub formula: String,
    #[serde(rename = "applyMod")]
    pub apply_mod: bool,
}

impl Damage {
    #[allow(unused)]
    pub fn without_mod(formula: String) -> Self {
        Damage {
            formula,
            apply_mod: false,
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct DamageScaling {
    pub formula: String,
    pub mode: DamageScalingMode,
}

#[derive(PartialEq, Debug)]
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

#[derive(Debug, PartialEq)]
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
    Poison,
    Positive,
    Slashing,
    Sonic,
    TempHp,
    None,
}

impl_deser! {
    DamageType :
    "acid" => DamageType::Acid,
    "bleed" => DamageType::Bleed,
    "bludgeoning" => DamageType::Bludgeoning,
    "cold" => DamageType::Cold,
    "chaotic" => DamageType::Chaotic,
    "electricity" => DamageType::Electricity,
    "evil" => DamageType::Evil,
    "fire" => DamageType::Fire,
    "force" => DamageType::Force,
    "good" => DamageType::Good,
    "healing" => DamageType::Healing,
    "lawful" => DamageType::Lawful,
    "mental" => DamageType::Mental,
    "negative" => DamageType::Negative,
    "piercing" => DamageType::Piercing,
    "poison" => DamageType::Poison,
    "positive" => DamageType::Positive,
    "slashing" => DamageType::Slashing,
    "sonic" => DamageType::Sonic,
    "temphp" => DamageType::TempHp,
    "" => DamageType::None,
    expects: "(acid|bleed|bludgeoning|chaotic|cold|electricity|evil|fire|force|good|healing|lawful|mental|negative|piercing|poison|positive|slashing|sonic|temphp|)"
}
