use serde::{Deserialize, Serialize};

use super::{size::Size, ValueWrapper};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(from = "JsonCreature")]
pub struct Creature {
    name: String,
    size: Size,
    ability_scores: AbilityModifiers,
    ac: i32,
    hp: i32,
    perception: i32,
    speeds: Speeds,
    alignment: Alignment,
    flavor_text: String,
    level: i32,
    source: String,
    saves: SavingThrows,
}

impl From<JsonCreature> for Creature {
    fn from(jc: JsonCreature) -> Self {
        Creature {
            name: jc.name,
            size: Size::Tiny,
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
            speeds: Speeds::from(jc.data.attributes.speed),
            alignment: jc.data.details.alignment.value,
            flavor_text: jc.data.details.flavor_text,
            level: jc.data.details.level.value,
            source: jc.data.details.source.value,
            saves: SavingThrows {
                reflex: jc.data.saves.reflex.value,
                fortitude: jc.data.saves.fortitude.value,
                will: jc.data.saves.will.value,
                additional_save_modifier: jc.data.attributes.all_saves.map(|v| v.value),
            },
        }
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
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, Eq)]
pub struct AbilityModifiers {
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
pub struct SavingThrows {
    reflex: i32,
    fortitude: i32,
    will: i32,
    additional_save_modifier: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, Eq)]
pub struct Speeds {
    land: i32,
    fly: Option<i32>,
    swim: Option<i32>,
    burrow: Option<i32>,
}

impl From<JsonCreatureSpeed> for Speeds {
    fn from(js: JsonCreatureSpeed) -> Self {
        fn find_speed(speeds: &JsonCreatureSpeed, t: SpeedType) -> Option<i32> {
            speeds
                .other_speeds
                .iter()
                .find_map(|s| (s.speed_type == t).then(|| &s.value))
                .and_then(|s| s.strip_suffix(" feet"))
                .and_then(|s| s.parse().ok())
        }

        Speeds {
            land: js.value.strip_suffix(" feet").and_then(|s| s.parse().ok()).expect("bad speed?"),
            fly: find_speed(&js, SpeedType::Fly),
            swim: find_speed(&js, SpeedType::Swim),
            burrow: find_speed(&js, SpeedType::Burrow),
        }
    }
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
    speed: JsonCreatureSpeed,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct JsonCreatureSpeed {
    value: String,
    other_speeds: Vec<OtherCreatureSpeed>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct OtherCreatureSpeed {
    #[serde(rename = "type")]
    speed_type: SpeedType,
    value: String,
}

#[derive(Deserialize, Debug, PartialEq)]
enum SpeedType {
    Land,
    Fly,
    Swim,
    Burrow,
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
        assert_eq!(
            dargon.speeds,
            Speeds {
                land: 60,
                fly: Some(180),
                burrow: None,
                swim: None,
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
        assert_eq!(dargon.alignment, Alignment::CE);
    }
}
