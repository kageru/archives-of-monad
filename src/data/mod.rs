use serde::{Deserialize, Deserializer};

pub mod actions;
pub mod ancestries;
pub mod conditions;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ValueWrapper<T> {
    value: T,
}

#[derive(Debug, PartialEq)]
pub enum AbilityScore {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl<'de> Deserialize<'de> for AbilityScore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "str" => Ok(AbilityScore::Strength),
            "dex" => Ok(AbilityScore::Dexterity),
            "con" => Ok(AbilityScore::Constitution),
            "int" => Ok(AbilityScore::Intelligence),
            "wis" => Ok(AbilityScore::Wisdom),
            "cha" => Ok(AbilityScore::Charisma),
            s => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(s),
                &"str|dex|con|int|wis|cha",
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
