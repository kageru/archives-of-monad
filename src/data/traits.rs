use super::creature::Alignment;
use super::equipment::ItemUsage;
use super::size::Size;
use super::ValueWrapper;
use crate::html::{write_full_page, Page};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt;
use std::{fs, io, io::BufReader};

lazy_static! {
    static ref TRAIT_PARAMETER_REGEX: Regex = Regex::new(r"-?(\d*[dD])?\d+$").unwrap();
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Trait {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq)]
#[serde(from = "JsonTraits")]
pub struct Traits {
    pub misc: Vec<String>,
    pub rarity: Rarity,
    pub alignment: Option<Alignment>,
    pub size: Option<Size>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct JsonTraits {
    pub value: Vec<String>,
    pub rarity: Option<ValueWrapper<Rarity>>,
    pub usage: Option<ValueWrapper<ItemUsage>>,
}

pub fn clean_trait_name(name: &str) -> String {
    match name {
        // we get these lowercase from items but uppercase from the trait descriptions
        n if n.starts_with("versatile") => String::from("versatile"),
        n if n.starts_with("Versatile") => String::from("Versatile"),
        n if n.starts_with("splash") || n.starts_with("Splash") => String::from(n),
        n => TRAIT_PARAMETER_REGEX.replace(n, "").to_string(),
    }
}

impl From<JsonTraits> for Traits {
    fn from(jt: JsonTraits) -> Self {
        let rarity = jt.rarity.map(|r| r.value);
        Traits {
            misc: jt.value,
            rarity: rarity.unwrap_or(Rarity::Common),
            size: None,
            alignment: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq, AsRefStr)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Unique,
}

impl Rarity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Rarity::Common => "Common",
            Rarity::Uncommon => "Uncommon",
            Rarity::Rare => "Rare",
            Rarity::Unique => "Unique",
        }
    }
}

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.as_str())
    }
}

pub struct TraitDescriptions(pub(crate) BTreeMap<String, String>);

pub(crate) fn read_trait_descriptions(path: &str) -> TraitDescriptions {
    let f = std::fs::File::open(path).expect("File missing");
    let reader = BufReader::new(f);
    let raw: Value = serde_json::from_reader(reader).expect("Deserialization failed");
    TraitDescriptions(
        raw["PF2E"]
            .as_object()
            .expect("Expected field PF2E to be present")
            .into_iter()
            .filter_map(|(k, v)| k.strip_prefix("TraitDescription").zip(v.as_str()))
            .map(|(k, v)| (clean_trait_name(k), v.to_owned()))
            .collect(),
    )
}

// These work differently from the other data structures because they’re not deserialized from a
// folder of JSONs.
pub(crate) fn render_traits(output_path: &str, descriptions: &TraitDescriptions) -> io::Result<Vec<Page>> {
    fs::create_dir_all(output_path)?;
    let mut list = String::with_capacity(100_000);
    list.push_str("<div id=\"gridlist\">");
    let mut pages = Vec::new();
    for (key, val) in &descriptions.0 {
        let trait_name = key.to_lowercase();
        let page = Page {
            name: key.to_string(),
            content: format!("<h1><a href=\"/trait/{}\">{}</a></h1><hr/>{}", trait_name, key, val),
            category: String::from("trait"),
            id: format!("trait-{}", key),
        };
        write_full_page(&format!("{}/{}", output_path, trait_name), &page.name, &page.content)?;
        list.push_str(&format!("<span><a href=\"{}\">{}</a></span>\n", trait_name, key));
        pages.push(page);
    }
    list.push_str("</div>");
    list.push_str("<div style=\"height: 2em\"></div>");
    list.push_str("<a href=\"/\">Back</a>");
    write_full_page(&format!("{}/index.html", output_path), "Traits", &list)?;
    Ok(pages)
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

    #[test]
    fn test_parameter_stripping() {
        assert_eq!("You can throw this weapon as a ranged attack. A thrown weapon adds your Strength modifier to damage just like a melee weapon does. When this trait appears on a melee weapon, it also includes the range increment.", DESCRIPTIONS.0["Thrown"]);
        assert_eq!("The fatal trait includes a die size. On a critical hit, the weapon's damage die increases to that die size instead of the normal die size, and the weapon adds one additional damage die of the listed size.", DESCRIPTIONS.0["Fatal"]);
        assert_eq!(None, DESCRIPTIONS.0.get("Thrown10"));
        assert_eq!(None, DESCRIPTIONS.0.get("FatalD8"));
    }
}
