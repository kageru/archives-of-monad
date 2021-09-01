use super::{
    damage::{DamageType, Die, EquipmentDamage},
    traits::{JsonTraits, Traits},
    HasName, ValueWrapper,
};
use crate::text_cleanup;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, cmp::Ordering, fmt, fmt::Display};

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
    pub price: String, // e.g. "2 sp"
    pub range: i32,
    pub splash_damage: i32,
    pub traits: Traits,
    pub usage: Option<ItemUsage>,
    pub weapon_type: WeaponType,
    pub weight: Weight,
    pub item_type: ItemType,
    pub value: Option<Money>,
}

impl Equipment {
    pub fn format_price(&self) -> Option<Cow<'_, str>> {
        if let Some(value) = &self.value {
            Some(Cow::Owned(format!("{} {}", value.value, value.currency)))
        } else if !self.price.is_empty() && !self.price.starts_with('0') {
            Some(Cow::Borrowed(&self.price))
        } else {
            None
        }
    }
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Copy)]
pub struct Money {
    pub value: i32,
    pub currency: Currency,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Display, Copy)]
#[allow(non_camel_case_types)]
pub enum Currency {
    pp,
    gp,
    sp,
    cp,
}

#[derive(Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(untagged)]
pub enum StringOrNum {
    String(String),
    Numerical(i32),
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
}

impl Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Weight::Bulk(n) => write!(f, "{} bulk", n),
            Weight::Light => write!(f, "light"),
            Weight::Negligible => write!(f, "negligible"),
        }
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(untagged)]
enum JsonWeight {
    String(String),
    Numerical(i32),
}

impl From<JsonWeight> for Weight {
    fn from(raw: JsonWeight) -> Self {
        match raw {
            JsonWeight::String(x) if x == "L" => Weight::Light,
            JsonWeight::String(x) if x == "-" => Weight::Negligible,
            JsonWeight::String(n) => Weight::Bulk(n.parse().unwrap()),
            JsonWeight::Numerical(n) => Weight::Bulk(n),
        }
    }
}

impl From<JsonEquipment> for Equipment {
    fn from(je: JsonEquipment) -> Self {
        Equipment {
            name: je.name.clone(),
            damage: je.data.damage.map(EquipmentDamage::from),
            description: text_cleanup(&je.data.description.value, true),
            group: je.data.group.and_then(|v| v.value).unwrap_or(WeaponGroup::NotAWeapon),
            hardness: je.data.hardness.value.unwrap_or(0),
            max_hp: je.data.max_hp.value.unwrap_or(0),
            level: je.data.level.value,
            price: je.data.price.value.into(),
            range: je.data.range.and_then(|v| v.value).and_then(|r| r.parse().ok()).unwrap_or(0),
            splash_damage: je.data.splash_damage.value.map(|v| v.into()).unwrap_or(0),
            usage: je.data.traits.usage.map(|v| v.value),
            traits: Traits::from(je.data.traits),
            weapon_type: je.data.weapon_type.map(|v| v.value).unwrap_or(WeaponType::NotAWeapon),
            weight: je.data.weight.value.into(),
            item_type: je.item_type,
            value: je.data.value.zip(je.data.denomination).map(|(v, c)| Money {
                value: v.value,
                currency: c.value,
            }),
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
    data: JsonEquipmentData,
    name: String,
    #[serde(rename = "type")]
    item_type: ItemType,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct JsonEquipmentData {
    damage: Option<JsonEquipmentDamage>,
    description: ValueWrapper<String>,
    group: Option<ValueWrapper<Option<WeaponGroup>>>,
    #[serde(default)]
    hardness: ValueWrapper<Option<i32>>,
    #[serde(default)]
    max_hp: ValueWrapper<Option<i32>>,
    #[serde(default)]
    level: ValueWrapper<i32>,
    price: ValueWrapper<StringOrNum>,
    // wtf
    range: Option<ValueWrapper<Option<String>>>,
    #[serde(default)]
    splash_damage: ValueWrapper<Option<StringOrNum>>,
    traits: JsonTraits,
    weapon_type: Option<ValueWrapper<WeaponType>>,
    weight: ValueWrapper<JsonWeight>,
    value: Option<ValueWrapper<i32>>,
    denomination: Option<ValueWrapper<Currency>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone, Copy, Display)]
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
    EtchedOntoAWeapon,
    EtchedOntoArmor,
    Bonded,
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
    Worngarwment,
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone, Copy, Display)]
#[serde(rename_all = "lowercase")]
pub enum WeaponType {
    Unarmed,
    Simple,
    Martial,
    Advanced,
    #[serde(alias = "")]
    NotAWeapon,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone, Copy)]
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
    #[serde(alias = "")]
    NotAWeapon,
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
        assert_eq!("2 sp", dagger.price);
        assert_eq!(10, dagger.range);
        assert_eq!(vec!["agile", "finesse", "thrown-10", "versatile-s"], dagger.traits.value);
        assert_eq!(Some(Rarity::Common), dagger.traits.rarity);
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
        assert_eq!("4 gp", crystal.price);
        assert_eq!(Weight::Negligible, crystal.weight);
    }

    #[test]
    fn test_treasure_value() {
        let lusty_argonian_maid: Equipment =
            serde_json::from_str(&read_test_file("equipment.db/amphora-with-lavish-scenes.json")).expect("Deserialization failed");
        assert_eq!(
            Some(Money {
                value: 10,
                currency: Currency::gp,
            }),
            lusty_argonian_maid.value,
        );
    }
}
