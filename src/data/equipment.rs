use super::{
    damage::{DamageType, Die, EquipmentDamage},
    traits::{JsonTraits, Traits},
    HasName, ValueWrapper,
};
use crate::text_cleanup;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt, fmt::Display};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(from = "JsonEquipment")]
pub struct Equipment {
    pub name: String,
    pub damage: Option<EquipmentDamage>,
    pub description: String,
    pub group: WeaponGroup,
    pub hardness: i32,
    pub max_hp: i32,
    pub level: i32,
    pub price: Price,
    pub range: i32,
    pub splash_damage: i32,
    pub traits: Traits,
    pub usage: Option<ItemUsage>,
    pub category: ProficiencyGroup,
    pub weight: Weight,
    pub item_type: ItemType,
    pub source: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, Eq, Default)]
pub struct Price {
    #[serde(default)]
    cp: u32,
    #[serde(default)]
    sp: u32,
    #[serde(default)]
    gp: u32,
    #[serde(default)]
    pp: u32,
}

impl Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.pp != 0 {
            write!(f, "{} pp ", group_digits(self.pp))?;
        }
        if self.gp != 0 {
            write!(f, "{} gp ", group_digits(self.gp))?;
        }
        if self.sp != 0 {
            write!(f, "{} sp ", group_digits(self.sp))?;
        }
        if self.cp != 0 {
            write!(f, "{} cp ", group_digits(self.cp))?;
        }
        Ok(())
    }
}

#[allow(unstable_name_collisions)]
fn group_digits(n: u32) -> String {
    n.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .intersperse(&[b','])
        .flatten()
        .map(|&b| b as char)
        .collect()
}

impl From<StringOrNum> for String {
    fn from(s: StringOrNum) -> Self {
        match s {
            StringOrNum::String(s) => s,
            StringOrNum::Numerical(n) => n.to_string(),
        }
    }
}

impl From<StringOrNum> for i32 {
    fn from(s: StringOrNum) -> Self {
        match s {
            StringOrNum::String(s) => s.parse().unwrap_or(0),
            StringOrNum::Numerical(n) => n,
        }
    }
}

impl From<&StringOrNum> for i32 {
    fn from(s: &StringOrNum) -> Self {
        match s {
            StringOrNum::String(s) => s.parse().unwrap_or(0),
            StringOrNum::Numerical(n) => *n,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(untagged)]
pub enum StringOrNum {
    String(String),
    Numerical(i32),
}

impl Default for StringOrNum {
    fn default() -> Self {
        StringOrNum::Numerical(0)
    }
}

impl HasName for Equipment {
    fn name(&self) -> &str {
        &self.name
    }
}

impl PartialOrd for Equipment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Equipment {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.level.cmp(&other.level) {
            Ordering::Equal => self.name.cmp(&other.name),
            o => o,
        }
    }
}

#[derive(Serialize, PartialEq, Debug, Clone, Eq)]
pub enum Weight {
    Bulk(i32),
    Light,
    Negligible,
    NotApplicable,
}

impl Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Weight::Bulk(n) => write!(f, "{} bulk", n),
            Weight::Light => write!(f, "light"),
            Weight::Negligible => write!(f, "negligible"),
            Weight::NotApplicable => write!(f, "N/A"),
        }
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(untagged)]
enum JsonWeight {
    String(String),
    Numerical(i32),
}

impl From<Option<JsonWeight>> for Weight {
    fn from(raw: Option<JsonWeight>) -> Self {
        match raw {
            Some(JsonWeight::String(x)) if x == "L" => Weight::Light,
            Some(JsonWeight::String(x)) if x == "-" => Weight::Negligible,
            Some(JsonWeight::String(n)) => Weight::Bulk(n.parse().unwrap()),
            Some(JsonWeight::Numerical(n)) => Weight::Bulk(n),
            None => Weight::NotApplicable,
        }
    }
}

impl From<JsonEquipment> for Equipment {
    fn from(je: JsonEquipment) -> Self {
        Equipment {
            name: je.name.clone(),
            damage: je.system.damage.map(EquipmentDamage::from),
            description: text_cleanup(&je.system.description.value),
            group: je.system.group.and_then(WrappedOrNot::value).unwrap_or(WeaponGroup::NotAWeapon),
            hardness: je.system.hardness,
            max_hp: je.system.hp.map(|hp| hp.max).unwrap_or(0),
            level: je.system.level.value.into(),
            price: je.system.price.value,
            range: je.system.range.and_then(WrappedOrNot::value).map(i32::from).unwrap_or(0),
            splash_damage: je.system.splash_damage.value.map(|v| v.into()).unwrap_or(0),
            usage: je.system.traits.usage.map(|v| v.value),
            traits: Traits::from(je.system.traits),
            category: je.system.category.unwrap_or(ProficiencyGroup::NoProficiency),
            weight: je.system.weight.map(|v| v.value).into(),
            item_type: je.item_type,
            source: je.system.source.value,
        }
    }
}

impl From<JsonEquipmentDamage> for EquipmentDamage {
    fn from(jed: JsonEquipmentDamage) -> Self {
        EquipmentDamage {
            damage_type: jed.damage_type,
            die: jed.die,
            number_of_dice: jed.dice,
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
struct JsonEquipment {
    system: JsonEquipmentData,
    name: String,
    #[serde(rename = "type")]
    item_type: ItemType,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum WrappedOrNot<T> {
    Wrapped(ValueWrapper<T>),
    Value(T),
}

impl<T> WrappedOrNot<T> {
    fn value(self) -> T {
        match self {
            WrappedOrNot::Wrapped(ValueWrapper { value: t }) => t,
            WrappedOrNot::Value(t) => t,
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct JsonEquipmentData {
    damage: Option<JsonEquipmentDamage>,
    description: ValueWrapper<String>,
    group: Option<WrappedOrNot<Option<WeaponGroup>>>,
    #[serde(default)]
    hardness: i32,
    hp: Option<JsonHp>,
    #[serde(default)]
    level: ValueWrapper<StringOrNum>,
    #[serde(default)]
    price: ValueWrapper<Price>,
    // Real data only uses Option<i32>, but non-weapons still use the broken and inconsistent old format
    range: Option<WrappedOrNot<Option<StringOrNum>>>,
    #[serde(default)]
    splash_damage: ValueWrapper<Option<StringOrNum>>,
    traits: JsonTraits,
    category: Option<ProficiencyGroup>,
    weight: Option<ValueWrapper<JsonWeight>>,
    value: Option<ValueWrapper<i32>>,
    source: ValueWrapper<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone, Copy)]
struct JsonHp {
    max: i32,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone, Copy, IntoStaticStr, EnumIter, AsRefStr)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Consumable,
    Weapon,
    Equipment,
    Treasure,
    Armor,
    Backpack,
    Kit,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum ItemUsage {
    HeldInOneHand,
    HeldInTwoHands,
    AffixedToWeapon,
    AffixedToArmor,
    AffixedToAShield,
    AffixedToArmorOrAWeapon,
    EtchedOntoAWeapon,
    EtchedOntoArmor,
    Bonded,
    TattooedOnTheBody,
    EtchedOntoMeleeWeapon,
    Worn,
    // Not sure about this yet… maybe we can parse these from the localization file
    // and show useful descriptions somehow?
    Wornring,
    Wornshoes,
    Wornnecklace,
    Wornmask,
    Wornhorseshoes,
    Wornheadwear,
    Worngloves,
    Worngarment,
    Worneyepiece,
    Wornepaulet,
    Worncollar,
    Worncloak,
    Worncirclet,
    Wornbracers,
    Wornbelt,
    Wornamor,
    Wornarmbands,
    Wornanklets,
    Wornamulet,
    Wornbracelet,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone, Copy, AsRefStr)]
#[serde(rename_all = "lowercase")]
pub enum ProficiencyGroup {
    // Weapons
    Unarmed,
    Simple,
    Martial,
    Advanced,
    // Armor
    Shield,
    Unarmored,
    Light,
    Medium,
    Heavy,
    #[serde(alias = "")]
    NoProficiency,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone, Copy, AsRefStr)]
#[serde(rename_all = "lowercase")]
pub enum WeaponGroup {
    Knife,
    Bow,
    Sword,
    Axe,
    Flail,
    Club,
    Brawling,
    Shield,
    Sling,
    Spear,
    Bomb,
    Polearm,
    Composite,
    Dart,
    Hammer,
    Pick,
    Leather,
    Chain,
    Plate,
    Firearm,
    #[serde(alias = "")]
    NotAWeapon,
    Cloth,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct JsonEquipmentDamage {
    damage_type: DamageType,
    dice: i32,
    die: Die,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{data::traits::Rarity, tests::read_test_file};

    #[test]
    fn test_dagger_deserialization() {
        let dagger: Equipment = serde_json::from_str(&read_test_file("equipment.db/dagger.json")).expect("Deserialization failed");
        assert_eq!("Dagger", dagger.name);
        assert_eq!(
            Some(EquipmentDamage {
                damage_type: DamageType::Piercing,
                die: Die::D4,
                number_of_dice: 1,
            }),
            dagger.damage
        );
        assert_eq!(
            Price {
                sp: 2,
                ..Default::default()
            },
            dagger.price
        );
        assert_eq!(0, dagger.range); // thrown trait but no inherent range
        assert_eq!(vec!["agile", "finesse", "thrown-10", "versatile-s"], dagger.traits.misc);
        assert_eq!(Rarity::Common, dagger.traits.rarity);
        assert_eq!(0, dagger.level);
        assert_eq!(ItemType::Weapon, dagger.item_type);
        assert_eq!(Weight::Light, dagger.weight);
        assert_eq!(ItemType::Weapon, dagger.item_type);
    }

    #[test]
    fn test_potency_crystal_deserialization() {
        let crystal: Equipment =
            serde_json::from_str(&read_test_file("equipment.db/potency-crystal.json")).expect("Deserialization failed");
        assert_eq!("Potency Crystal", crystal.name);
        assert_eq!(1, crystal.level);
        assert_eq!(crystal.item_type, ItemType::Consumable);
        assert_eq!(
            Price {
                gp: 4,
                ..Default::default()
            },
            crystal.price
        );
        assert_eq!(Weight::Negligible, crystal.weight);
        assert_eq!("Pathfinder Core Rulebook", crystal.source);
    }

    #[test]
    fn test_treasure_value() {
        let lusty_argonian_maid: Equipment =
            serde_json::from_str(&read_test_file("equipment.db/amphora-with-lavish-scenes.json")).expect("Deserialization failed");
        assert_eq!(
            Price {
                gp: 10,
                ..Default::default()
            },
            lusty_argonian_maid.price,
        );
    }

    #[test]
    fn test_digit_grouping() {
        assert_eq!(group_digits(1), "1");
        assert_eq!(group_digits(10), "10");
        assert_eq!(group_digits(100), "100");
        assert_eq!(group_digits(1000), "1,000");
        assert_eq!(group_digits(12345), "12,345");
        assert_eq!(group_digits(33445566), "33,445,566");
    }
}
