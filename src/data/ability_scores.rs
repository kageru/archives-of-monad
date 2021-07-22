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

#[derive(Debug, PartialEq)]
pub struct AbilityBoost(pub Vec<AbilityScore>);

impl AbilityBoost {
    fn is_free(&self) -> bool {
        self.0.len() >= 6
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

#[derive(Deserialize, Debug, PartialEq)]
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
        let json = r#"{
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
