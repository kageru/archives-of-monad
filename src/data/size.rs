use crate::impl_deser;
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

impl_deser! {
    Size :
    "tiny" => Size::Tiny,
    "sm" => Size::Small,
    "med" => Size::Medium,
    "lg" => Size::Large,
    "huge" => Size::Huge,
    "grg" => Size::Gargantuan,
    expects: "tiny|sm|med|lg|huge|grg"
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
