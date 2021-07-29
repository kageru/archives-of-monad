use serde::{de, Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub enum Proficiency {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
}

impl<'de> Deserialize<'de> for Proficiency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match i32::deserialize(deserializer)? {
            0 => Ok(Proficiency::Untrained),
            1 => Ok(Proficiency::Trained),
            2 => Ok(Proficiency::Expert),
            3 => Ok(Proficiency::Master),
            4 => Ok(Proficiency::Legendary),
            s => Err(de::Error::invalid_value(de::Unexpected::Signed(s as i64), &"0|1|2|3|4")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::ValueWrapper;

    #[test]
    fn should_deserialize_size() {
        let json = r#"{ "value": 1 }"#;
        let prof: ValueWrapper<Proficiency> = serde_json::from_str(json).unwrap();
        assert_eq!(prof.value, Proficiency::Trained);
    }
}
