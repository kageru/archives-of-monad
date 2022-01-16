use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, AsRefStr)]
pub enum Vision {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "lowLightVision")]
    LowLightVision,
    #[serde(rename = "darkvision")]
    DarkVision,
}

impl Vision {
    pub fn is_normal(&self) -> bool {
        self == &Vision::Normal
    }

    pub fn name(&self) -> &str {
        match self {
            Vision::Normal => "Normal",
            Vision::LowLightVision => "Low-Light Vision",
            Vision::DarkVision => "Darkvision",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::ValueWrapper;

    #[test]
    fn should_deserialize_vision() {
        let json = r#"{ "value": "lowLightVision" }"#;
        let vision: ValueWrapper<Vision> = serde_json::from_str(json).unwrap();
        assert_eq!(vision.value, Vision::LowLightVision);
    }
}
