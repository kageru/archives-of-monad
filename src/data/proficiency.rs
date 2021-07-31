use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i32)]
pub enum Proficiency {
    Untrained = 0,
    Trained = 1,
    Expert = 2,
    Master = 3,
    Legendary = 4,
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
