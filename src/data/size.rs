use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Size {
    #[serde(rename = "tiny")]
    Tiny,
    #[serde(rename = "sm")]
    Small,
    #[serde(rename = "med")]
    Medium,
    #[serde(rename = "lg")]
    Large,
    #[serde(rename = "huge")]
    Huge,
    #[serde(rename = "grg")]
    Gargantuan,
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
