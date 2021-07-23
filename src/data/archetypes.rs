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
    use std::io::BufReader;

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

    #[test]
    fn should_deserialize_real_archetype() {
        let f = std::fs::File::open("tests/data/archetypes/assassin.json").expect("File missing");
        let reader = BufReader::new(f);
        let assassin: Archetype = serde_json::from_reader(reader).expect("Deserialization failed");
        assert_eq!(assassin.name, String::from("Assassin"));
    }
}
