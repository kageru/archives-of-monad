use super::{size::Size, traits::Rarity, HasLevel, ValueWrapper};
use crate::data::traits::Traits;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(from = "JsonCreature")]
pub struct Creature {
    pub name: String,
    pub ability_scores: AbilityModifiers,
    pub ac: i32,
    pub hp: i32,
    pub perception: i32,
    pub speeds: CreatureSpeeds,
    pub flavor_text: String,
    pub level: i32,
    pub source: String,
    pub saves: SavingThrows,
    pub traits: Traits,
}

impl From<JsonCreature> for Creature {
    fn from(jc: JsonCreature) -> Self {
        Creature {
            name: jc.name,
            ability_scores: AbilityModifiers {
                strength: jc.data.abilities.str.modifier,
                dexterity: jc.data.abilities.dex.modifier,
                constitution: jc.data.abilities.con.modifier,
                intelligence: jc.data.abilities.int.modifier,
                wisdom: jc.data.abilities.wis.modifier,
                charisma: jc.data.abilities.cha.modifier,
            },
            ac: jc.data.attributes.ac.value,
            hp: jc.data.attributes.hp.value,
            perception: jc.data.attributes.perception.value,
            speeds: jc.data.attributes.speed,
            flavor_text: jc.data.details.flavor_text,
            level: jc.data.details.level.value,
            source: jc.data.details.source.value,
            saves: SavingThrows {
                reflex: jc.data.saves.reflex.value,
                fortitude: jc.data.saves.fortitude.value,
                will: jc.data.saves.will.value,
                additional_save_modifier: jc.data.attributes.all_saves.map(|v| v.value),
            },
            traits: Traits {
                misc: vec![],
                rarity: Rarity::Common,
                size: Some(Size::Tiny),
                alignment: Some(jc.data.details.alignment.value),
            },
        }
    }
}

impl HasLevel for Creature {
    fn level(&self) -> i32 {
        self.level
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, AsRefStr, Clone, Copy)]
pub enum Alignment {
    LG,
    NG,
    CG,
    LN,
    N,
    CN,
    LE,
    NE,
    CE,
    // summons like an unseen servant are unaligned
    #[serde(other)]
    Unaligned,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, Eq)]
pub struct AbilityModifiers {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
pub struct SavingThrows {
    pub reflex: i32,
    pub fortitude: i32,
    pub will: i32,
    pub additional_save_modifier: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
pub struct Speeds {
    pub general: String,
    pub fly: Option<i32>,
    pub swim: Option<i32>,
    pub burrow: Option<i32>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreature {
    data: JsonCreatureData,
    name: String,
}

// also has items which include loot and… the prepared spells, spell DCs, etc. wtf?
#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreatureData {
    abilities: JsonCreatureAbilities,
    attributes: JsonCreatureAttributes,
    details: JsonCreatureDetails,
    saves: JsonCreatureSaves,
    traits: JsonCreatureTraits, // different from usual traits
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreatureAbilities {
    cha: JsonCreatureAbility,
    con: JsonCreatureAbility,
    dex: JsonCreatureAbility,
    int: JsonCreatureAbility,
    str: JsonCreatureAbility,
    wis: JsonCreatureAbility,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreatureAbility {
    #[serde(rename = "mod")]
    modifier: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JsonCreatureAttributes {
    ac: ValueWrapper<i32>,
    all_saves: Option<ValueWrapper<String>>,
    hp: ValueWrapper<i32>,
    perception: ValueWrapper<i32>,
    speed: CreatureSpeeds,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatureSpeeds {
    value: String,
    other_speeds: Vec<OtherCreatureSpeed>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OtherCreatureSpeed {
    #[serde(rename = "type")]
    speed_type: String,
    value: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JsonCreatureDetails {
    alignment: ValueWrapper<Alignment>,
    flavor_text: String,
    level: ValueWrapper<i32>,
    source: ValueWrapper<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonCreatureSaves {
    fortitude: ValueWrapper<i32>,
    reflex: ValueWrapper<i32>,
    will: ValueWrapper<i32>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JsonCreatureTraits {}

#[cfg(test)]
mod tests {
    use crate::tests::read_test_file;

    use super::*;

    #[test]
    fn test_deserialize_budget_dahak() {
        let dargon: Creature =
            serde_json::from_str(&read_test_file("pathfinder-bestiary.db/ancient-red-dragon.json")).expect("deserialization failed");
        assert_eq!(
            dargon.saves,
            SavingThrows {
                reflex: 32,
                fortitude: 35,
                will: 35,
                additional_save_modifier: Some("+1 status to all saves vs. magic".to_string()),
            }
        );
        assert_eq!(dargon.name, "Ancient Red Dragon".to_string());
        assert_eq!(dargon.perception, 35);
        assert_eq!(dargon.ac, 45);
        assert_eq!(dargon.hp, 425);
        assert_eq!(
            dargon.ability_scores,
            AbilityModifiers {
                strength: 9,
                dexterity: 5,
                constitution: 8,
                intelligence: 5,
                wisdom: 6,
                charisma: 7,
            }
        );
        assert_eq!(
            dargon.traits,
            Traits {
                misc: vec![],
                size: Some(Size::Tiny),
                alignment: Some(Alignment::CE),
                rarity: Rarity::Common,
            }
        );
    }
}