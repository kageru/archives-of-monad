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
}
