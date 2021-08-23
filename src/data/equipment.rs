use super::{
    damage::{DamageType, Die, EquipmentDamage},
    traits::{JsonTraits, Traits},
    ValueWrapper,
};
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(from = "JsonEquipment")]
struct Equipment {
    name: String,
    broken_threshold: i32,
    damage: Option<EquipmentDamage>,
    description: String,
    group: WeaponGroup,
    hands: Option<i32>,
    hardness: i32,
    max_hp: i32,
    level: i32,
    price: String, // e.g. "2 sp"
    range: i32,
    splash_damage: i32,
    traits: Traits,
    usage: Option<ItemUsage>,
    weapon_type: WeaponType,
    weight: Weight,
    item_type: ItemType,
}

#[derive(Deserialize, PartialEq, Debug, Clone, Eq)]
enum Weight {
    Bulk(i32),
    Light,
    Negligible,
}

impl From<&str> for Weight {
    fn from(raw: &str) -> Self {
        match raw {
            "L" => Weight::Light,
            "-" => Weight::Negligible,
            n => Weight::Bulk(n.parse().unwrap()),
        }
    }
}

impl From<JsonEquipment> for Equipment {
    fn from(je: JsonEquipment) -> Self {
        Equipment {
            name: je.name,
            broken_threshold: je.data.broken_threshold.value,
            damage: je.data.damage.map(EquipmentDamage::from),
            description: je.data.description.value,
            group: je.data.group.map(|v| v.value).unwrap_or(WeaponGroup::NotAWeapon),
            hands: je.data.hands.and_then(|h| h.value.parse().ok()),
            hardness: je.data.hardness.value,
            max_hp: je.data.max_hp.value,
            level: je.data.level.value,
            price: je.data.price.value,
            range: je.data.range.value.and_then(|r| r.parse().ok()).unwrap_or(0),
            splash_damage: je.data.splash_damage.value,
            usage: je.data.traits.usage.map(|v| v.value),
            traits: Traits::from(je.data.traits),
            weapon_type: je.data.weapon_type.map(|v| v.value).unwrap_or(WeaponType::NotAWeapon),
            weight: je.data.weight.value.as_str().into(),
            item_type: je.item_type,
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
    broken_threshold: ValueWrapper<i32>,
    damage: Option<JsonEquipmentDamage>,
    description: ValueWrapper<String>,
    group: Option<ValueWrapper<WeaponGroup>>,
    hands: Option<ValueWrapper<String>>,
    hardness: ValueWrapper<i32>,
    max_hp: ValueWrapper<i32>,
    level: ValueWrapper<i32>,
    price: ValueWrapper<String>,
    range: ValueWrapper<Option<String>>,
    #[serde(default)]
    splash_damage: ValueWrapper<i32>,
    traits: JsonTraits,
    weapon_type: Option<ValueWrapper<WeaponType>>,
    weight: ValueWrapper<String>,
}

#[derive(Deserialize, PartialEq, Debug, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Consumable,
    Weapon,
}

#[derive(Deserialize, PartialEq, Debug, Eq, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum ItemUsage {
    HeldInOneHand,
    AffixedToWeapon,
}

#[derive(Deserialize, PartialEq, Debug, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum WeaponType {
    Simple,
    Martial,
    Advanced,
    NotAWeapon,
}

#[derive(Deserialize, PartialEq, Debug, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum WeaponGroup {
    Knife,
    Bow,
    Sword,
    Axe,
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
}
