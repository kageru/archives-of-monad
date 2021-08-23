use super::ValueWrapper;
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt;
use std::{fs, io, io::BufReader};

#[derive(Debug, PartialEq, Clone)]
pub struct Trait {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(from = "JsonTraits")]
pub struct Traits {
    pub value: Vec<String>,
    pub rarity: Option<Rarity>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct JsonTraits {
    pub value: Vec<String>,
    pub rarity: Option<ValueWrapper<Rarity>>,
}

impl From<JsonTraits> for Traits {
    fn from(jt: JsonTraits) -> Self {
        let rarity = jt.rarity.map(|r| r.value);
        Traits { value: jt.value, rarity }
    }
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Unique,
}

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Rarity::Common => "Common",
            Rarity::Uncommon => "Uncommon",
            Rarity::Rare => "Rare",
            Rarity::Unique => "Unique",
        };
        write!(f, "{}", s)
    }
}

pub struct TraitDescriptions(pub(crate) BTreeMap<String, String>);

pub fn read_trait_descriptions(path: &str) -> TraitDescriptions {
    let f = std::fs::File::open(path).expect("File missing");
    let reader = BufReader::new(f);
    let raw: Value = serde_json::from_reader(reader).expect("Deserialization failed");
    TraitDescriptions(
        raw["PF2E"]
            .as_object()
            .expect("Expected field PF2E to be present")
            .into_iter()
            .filter_map(|(k, v)| k.strip_prefix("TraitDescription").zip(v.as_str()))
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect(),
    )
}

// These work differently from the other data structures because they’re not deserialized from a
// folder of JSONs.
pub fn render_traits(output_path: &str, descriptions: &TraitDescriptions) -> io::Result<()> {
    fs::create_dir_all(output_path)?;
    let mut list = String::with_capacity(100_000);
    list.push_str("<div id=\"gridlist\">");
    for (key, val) in &descriptions.0 {
        let trait_page = format!("<h1>{}</h1><hr/>{}", key, val);
        let trait_name = key.to_lowercase();
        let full_output_filename = &format!("{}/{}", output_path, trait_name);
        fs::write(full_output_filename, trait_page)?;
        list.push_str(&format!("<span><a href=\"{}\">{}</a></span>\n", trait_name, trait_name));
    }
    list.push_str("</div>");
    list.push_str("<div style=\"height: 2em\"></div>");
    list.push_str("<a href=\"/\">Back</a>");
    fs::write(&format!("{}/index.html", output_path), &list)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::ValueWrapper;
    use crate::tests::DESCRIPTIONS;

    #[test]
    fn should_deserialize_rarity() {
        let json = r#"{ "value": "rare" }"#;
        let size: ValueWrapper<Rarity> = serde_json::from_str(json).unwrap();
        assert_eq!(size.value, Rarity::Rare);
    }

    #[test]
    fn test_trait_descriptions() {
        assert_eq!(
            String::from("A creature with this trait is a member of the aasimar ancestry."),
            DESCRIPTIONS.0["Aasimar"]
        );
        assert_eq!(
            String::from("A mental effect can alter the target's mind. It has no effect on an object or a mindless creature."),
            DESCRIPTIONS.0["Mental"]
        );
        assert_eq!(None, DESCRIPTIONS.0.get("some other key"));
    }
}
