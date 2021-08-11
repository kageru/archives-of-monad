use askama::Template;
use serde::Deserialize;

use super::HasName;

#[derive(Deserialize, Debug, PartialEq, Template, Clone)]
#[template(path = "deity.html", escape = "none")]
pub struct Deity {
    pub content: String,
    pub name: String,
}

impl HasName for Deity {
    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod test {
    use crate::tests::read_test_file;

    use super::*;
    use serde_json::json;

    #[test]
    fn should_deserialize_deity() {
        let json = json!(
        {
            "content": "Testing",
            "name": "Tester"
        });

        let deity: Deity = serde_json::from_value(json).unwrap();
        assert_eq!(
            deity,
            Deity {
                name: "Tester".into(),
                content: "Testing".into(),
            }
        );
    }

    #[test]
    fn should_deserialize_real_deity() {
        let asmodeus: Deity = serde_json::from_str(&read_test_file("deities.db/asmodeus.json")).expect("Deserialization failed");
        assert_eq!(asmodeus.name, String::from("Asmodeus"));
    }
}
