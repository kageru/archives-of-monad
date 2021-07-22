use serde::{de, Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

impl<'de> Deserialize<'de> for Size {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "tiny" => Ok(Size::Tiny),
            "sm" => Ok(Size::Small),
            "med" => Ok(Size::Medium),
            "lg" => Ok(Size::Large),
            "huge" => Ok(Size::Huge),
            "grg" => Ok(Size::Gargantuan),
            s => Err(de::Error::invalid_value(de::Unexpected::Str(s), &"tiny|sm|med|lg|huge|grg")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::ValueWrapper;

    #[test]
    fn should_deserialize_size() {
        let json = r#"{ "value": "grg" }"#;
        let size: ValueWrapper<Size> = serde_json::from_str(json).unwrap();
        assert_eq!(size.value, Size::Gargantuan);
    }
}
