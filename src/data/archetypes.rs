use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Archetype {
    content: String,
    name: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_deserialize_archetype() {
        let json = json!(
        {
            "content": "Testing",
            "name": "Tester"
        })
        .to_string();

        let archetype: Archetype = serde_json::from_str::<Archetype>(&json).unwrap();
        assert_eq!(
            archetype,
            Archetype {
                name: "Tester".into(),
                content: "Testing".into(),
            }
        );
    }
}
