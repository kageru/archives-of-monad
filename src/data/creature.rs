use super::{size::Size, traits::Rarity, HasLevel, ValueWrapper};
use crate::data::traits::Traits;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
#[serde(from = "JsonCreature")]
pub struct Creature {
    pub name: String,
    pub ability_scores: AbilityModifiers,
    pub ac: i32,
    pub ac_details: Option<String>,
    pub hp: i32,
    pub hp_details: Option<String>,
    pub perception: i32,
    pub senses: String,
    pub speeds: CreatureSpeeds,
    pub flavor_text: String,
    pub level: i32,
    pub source: String,
    pub saves: SavingThrows,
    pub traits: Traits,
    pub resistances: Vec<(String, Option<i32>)>,
    pub weaknesses: Vec<(String, Option<i32>)>,
    pub immunities: Vec<String>,
    pub languages: Vec<String>,
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
            ac_details: Some(jc.data.attributes.ac.details).filter(|d| !d.is_empty()),
            hp: jc.data.attributes.hp.value,
            hp_details: Some(jc.data.attributes.hp.details).filter(|d| !d.is_empty()),
            perception: jc.data.attributes.perception.value,
            senses: match jc.data.traits.senses {
                StringWrapperOrList::Wrapper(w) => w.value,
                StringWrapperOrList::List(l) => l.join(", "),
            },
            speeds: jc.data.attributes.speed.ensure_trailing_unit(),
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
                misc: titlecased(&jc.data.traits.traits.value),
                rarity: jc.data.traits.rarity.value,
                size: Some(jc.data.traits.size.value),
                alignment: Some(jc.data.details.alignment.value),
            },
            resistances: jc
                .data
                .traits
                .dr
                .into_iter()
                .map(|dr| (dr.label, dr.value.map(|v| v.parse().expect("Bad resistance value"))))
                .collect(),
            weaknesses: jc
                .data
                .traits
                .dv
                .into_iter()
                .map(|dv| (dv.label, dv.value.map(|v| v.parse().expect("Bad weakness value"))))
                .collect(),
            immunities: lowercased(&jc.data.traits.di.value),
            languages: {
                let mut titlecased = titlecased(&jc.data.traits.languages.value);
                if !jc.data.traits.languages.custom.is_empty() {
                    titlecased.push(jc.data.traits.languages.custom.from_case(Case::Lower).to_case(Case::Title));
                }
                titlecased
            },
        }
    }
}

fn ensure_trailing_unit(speed: &str) -> String {
    let speed = speed.trim();
    if speed.ends_with(" feet") {
        speed.to_string()
    } else {
        format!("{} feet", speed)
    }
}

fn titlecased(xs: &[String]) -> Vec<String> {
    xs.iter()
        .filter(|&l| l != "custom")
        .map(|l| l.from_case(Case::Kebab).to_case(Case::Title))
        .collect()
}

fn lowercased(xs: &[String]) -> Vec<String> {
    xs.iter()
        .filter(|&l| l != "custom")
        .map(|l| l.from_case(Case::Kebab).to_case(Case::Lower))
        .collect()
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
    ac: AcHpDetails,
    all_saves: Option<ValueWrapper<String>>,
    hp: AcHpDetails,
    perception: ValueWrapper<i32>,
    speed: CreatureSpeeds,
}

#[derive(Deserialize, Debug, PartialEq)]
struct AcHpDetails {
    value: i32,
    details: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatureSpeeds {
    pub value: String,
    pub other_speeds: Vec<OtherCreatureSpeed>,
}

impl CreatureSpeeds {
    fn ensure_trailing_unit(self) -> Self {
        CreatureSpeeds {
            value: ensure_trailing_unit(&self.value),
            other_speeds: self
                .other_speeds
                .into_iter()
                .map(|speed| OtherCreatureSpeed {
                    speed_type: speed.speed_type,
                    value: ensure_trailing_unit(&speed.value),
                })
                .collect(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct OtherCreatureSpeed {
    #[serde(rename = "type")]
    pub speed_type: String,
    pub value: String,
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
struct JsonCreatureTraits {
    rarity: ValueWrapper<Rarity>,
    senses: StringWrapperOrList,
    size: ValueWrapper<Size>,
    // yes, traits inside the traits. amazing, I know
    traits: ValueWrapper<Vec<String>>,
    languages: JsonLanguages,
    // I think this means damage immunities, but there are sometimes conditions in it.
    // There’s also ci which I assume would be where condition immunities actually belong.
    di: ValueWrapper<Vec<String>>,
    dv: Vec<JsonResistanceOrWeakness>,
    dr: Vec<JsonResistanceOrWeakness>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonLanguages {
    custom: String,
    value: Vec<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum StringWrapperOrList {
    Wrapper(ValueWrapper<String>),
    List(Vec<String>),
}

#[derive(Deserialize, PartialEq, Debug)]
struct JsonResistanceOrWeakness {
    label: String,
    value: Option<String>,
}

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
                misc: vec!["Dragon".to_string(), "Fire".to_string()],
                size: Some(Size::Huge),
                alignment: Some(Alignment::CE),
                rarity: Rarity::Uncommon,
            }
        );
        assert_eq!(dargon.senses, "darkvision, scent (imprecise) 60 feet, smoke vision");
        assert_eq!(dargon.weaknesses, vec![("Cold".to_string(), Some(20))]);
        assert_eq!(
            dargon.immunities,
            vec!["fire".to_string(), "paralyzed".to_string(), "sleep".to_string()]
        );
        assert_eq!(
            dargon.languages,
            vec![
                "Abyssal".to_string(),
                "Common".to_string(),
                "Draconic".to_string(),
                "Dwarven".to_string(),
                "Jotun".to_string(),
                "Orcish".to_string(),
            ]
        );
    }
}
