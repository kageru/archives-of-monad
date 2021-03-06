use super::ValueWrapper;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, PartialEq, IntoStaticStr, Clone, Copy, Eq, AsRefStr)]
pub enum AbilityScore {
    #[serde(rename = "str")]
    Strength,
    #[serde(rename = "dex")]
    Dexterity,
    #[serde(rename = "con")]
    Constitution,
    #[serde(rename = "int")]
    Intelligence,
    #[serde(rename = "wis")]
    Wisdom,
    #[serde(rename = "cha")]
    Charisma,
}

#[derive(Serialize, Debug, PartialEq, Clone, Eq)]
pub struct AbilityBoost(pub Vec<AbilityScore>);

impl Display for AbilityBoost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_free() {
            write!(f, "free")
        } else {
            write!(f, "{}", self.0.iter().map_into::<&str>().join(" or "))
        }
    }
}

impl AbilityBoost {
    pub fn is_free(&self) -> bool {
        self.0.len() >= 6
    }

    pub fn name(&self) -> Option<String> {
        if self.is_free() {
            Some("Free".to_string())
        } else {
            self.0.first().map(|b| b.as_ref().to_string())
        }
    }
}

impl From<JsonAbilityBoosts> for Vec<AbilityBoost> {
    fn from(jb: JsonAbilityBoosts) -> Self {
        let mut v = Vec::with_capacity(3);
        if let Some(f) = jb.first {
            v.push(AbilityBoost(f.value));
        }
        if let Some(s) = jb.second {
            v.push(AbilityBoost(s.value));
        }
        if let Some(t) = jb.third {
            v.push(AbilityBoost(t.value));
        }
        v
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct JsonAbilityBoosts {
    #[serde(rename = "0")]
    first: Option<ValueWrapper<Vec<AbilityScore>>>,
    #[serde(rename = "1")]
    second: Option<ValueWrapper<Vec<AbilityScore>>>,
    #[serde(rename = "2")]
    third: Option<ValueWrapper<Vec<AbilityScore>>>,
}

#[cfg(test)]
mod test {
    use super::*;
    const JSON: &str = r#"
{
    "0": {
        "value": [
            "cha"
        ]
    },
    "1": {
        "value": [
            "con"
        ]
    },
    "2": {
        "value": [
            "str",
            "dex",
            "con",
            "int",
            "wis",
            "cha"
        ]
    }
}"#;

    #[test]
    fn should_deserialize_ability_boosts() {
        let boosts: JsonAbilityBoosts = serde_json::from_str(JSON).expect("Deserialization failed");
        assert_eq!(
            boosts,
            JsonAbilityBoosts {
                first: Some(vec![AbilityScore::Charisma].into()),
                second: Some(vec![AbilityScore::Constitution].into()),
                third: Some(
                    vec![
                        AbilityScore::Strength,
                        AbilityScore::Dexterity,
                        AbilityScore::Constitution,
                        AbilityScore::Intelligence,
                        AbilityScore::Wisdom,
                        AbilityScore::Charisma,
                    ]
                    .into()
                ),
            }
        )
    }

    #[test]
    fn should_convert_into_usable_ability_scores() {
        let boosts: Vec<AbilityBoost> = serde_json::from_str::<JsonAbilityBoosts>(JSON)
            .expect("Deserialization failed")
            .into();
        assert_eq!(
            boosts,
            vec![
                AbilityBoost(vec![AbilityScore::Charisma]),
                AbilityBoost(vec![AbilityScore::Constitution]),
                AbilityBoost(vec![
                    AbilityScore::Strength,
                    AbilityScore::Dexterity,
                    AbilityScore::Constitution,
                    AbilityScore::Intelligence,
                    AbilityScore::Wisdom,
                    AbilityScore::Charisma,
                ]),
            ]
        );
        assert!(!boosts[1].is_free());
        assert!(boosts[2].is_free());
    }

    #[test]
    fn should_deserialize_single_ability_flaw() {
        let json = r#"
        {
            "0": {
                "value": [
                    "str"
                ]
            }
        }"#;
        let flaws: JsonAbilityBoosts = serde_json::from_str(json).unwrap();
        assert_eq!(
            flaws,
            JsonAbilityBoosts {
                first: Some(vec![AbilityScore::Strength].into()),
                second: None,
                third: None,
            }
        );
    }

    #[test]
    fn should_deserialize_ability_scores() {
        assert_eq!(
            serde_json::from_str::<ValueWrapper<AbilityScore>>(r#"{ "value": "str" }"#).unwrap(),
            ValueWrapper {
                value: AbilityScore::Strength
            }
        );
        assert_eq!(
            serde_json::from_str::<ValueWrapper<AbilityScore>>(r#"{ "value": "int" }"#).unwrap(),
            ValueWrapper {
                value: AbilityScore::Intelligence
            }
        );
    }
}
