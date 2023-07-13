use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
pub struct Condition {
    pub name: String,
    pub description: String,
    pub source: String,
    pub url: String,
}

#[cfg(test)]
mod test {
    use crate::tests::read_scraped_file;

    use super::*;

    #[test]
    fn should_deserialize_conditions() {
        let conditions: Vec<Condition> = serde_json::from_str(&read_scraped_file("conditions")).expect("Deserialization failed");
        assert_eq!(conditions[0].name, String::from("Blinded"));
    }
}
