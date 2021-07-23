use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Deity {
    content: String,
    name: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn should_deserialize_deity() {
        let json = json!(
        {
            "content": "Testing",
            "name": "Tester"
        })
        .to_string();

        let deity: Deity = serde_json::from_str::<Deity>(&json).unwrap();
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
        let f = std::fs::File::open("tests/data/deities/asmodeus.json").expect("File missing");
        let reader = BufReader::new(f);
        let asmodeus: Deity = serde_json::from_reader(reader).expect("Deserialization failed");
        assert_eq!(asmodeus.name, String::from("Asmodeus"));
    }
}
