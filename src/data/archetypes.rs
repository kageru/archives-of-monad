use crate::replace_references;
use askama::Template;
use serde::Deserialize;

use super::HasName;

#[derive(Deserialize, Debug, PartialEq, Template, Clone)]
#[template(path = "archetype.html", escape = "none")]
#[serde(from = "JsonArchetype")]
pub struct Archetype {
    pub content: String,
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsonArchetype {
    content: String,
    name: String,
}

impl From<JsonArchetype> for Archetype {
    fn from(ja: JsonArchetype) -> Self {
        Archetype {
            content: replace_references(&ja.content),
            name: ja.name,
        }
    }
}

impl HasName for Archetype {
    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::read_test_file;
    use serde_json::json;

    #[test]
    fn should_deserialize_archetype() {
        let json = json!(
        {
            "content": "Testing",
            "name": "Tester"
        });

        let archetype: Archetype = serde_json::from_value(json).unwrap();
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
        let assassin: Archetype = serde_json::from_str(&read_test_file("archetypes.db/assassin.json")).expect("Deserialization failed");
        assert_eq!(assassin.name, String::from("Assassin"));
    }
}
