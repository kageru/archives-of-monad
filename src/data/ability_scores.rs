use super::ValueWrapper;
use serde::{de, Deserialize, Deserializer};

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
            s => Err(de::Error::invalid_value(de::Unexpected::Str(s), &"str|dex|con|int|wis|cha")),
        }
    }
}

trait AbilityBoost {
    fn is_free(&self) -> bool;
}

impl AbilityBoost for Vec<AbilityBoosts> {
    fn is_free(&self) -> bool {
        self.len() == 6
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct AbilityBoosts {
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

    #[test]
    fn should_deserialize_ability_boosts() {
        let json = r#"
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
        let boosts: AbilityBoosts = serde_json::from_str(json).unwrap();
        assert_eq!(
            boosts,
            AbilityBoosts {
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
    fn should_deserialize_single_ability_flaw() {
        let json = r#"{
    "0": {
        "value": [
            "str"
        ]
    }
}"#;
        let flaws: AbilityBoosts = serde_json::from_str(json).unwrap();
        assert_eq!(
            flaws,
            AbilityBoosts {
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
