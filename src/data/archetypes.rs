use askama::Template;
use serde::Deserialize;

use super::HasName;

#[derive(Deserialize, Debug, PartialEq, Template, Clone)]
#[template(path = "archetype.html", escape = "none")]
pub struct Archetype {
    pub content: String,
    pub name: String,
}

impl HasName for Archetype {
    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
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
