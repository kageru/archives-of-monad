use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, Debug, PartialEq, Display)]
pub enum Skill {
    #[serde(rename = "acr")]
    Acrobatics,
    #[serde(rename = "arc")]
    Arcana,
    #[serde(rename = "ath")]
    Athletics,
    #[serde(rename = "cra")]
    Crafting,
    #[serde(rename = "dec")]
    Deception,
    #[serde(rename = "dip")]
    Diplomacy,
    #[serde(rename = "itm")]
    Intimidation,
    #[serde(rename = "med")]
    Medicine,
    #[serde(rename = "nat")]
    Nature,
    #[serde(rename = "occ")]
    Occultism,
    #[serde(rename = "prf")]
    Performance,
    #[serde(rename = "rel")]
    Religion,
    #[serde(rename = "soc")]
    Society,
    #[serde(rename = "ste")]
    Stealth,
    #[serde(rename = "sur")]
    Survival,
    #[serde(rename = "thi")]
    Thievery,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::ValueWrapper;

    #[test]
    fn should_deserialize_size() {
        let json = r#"{ "value": ["occ"] }"#;
        let skill: ValueWrapper<Vec<Skill>> = serde_json::from_str(json).unwrap();
        assert_eq!(skill.value[0], Skill::Occultism);
    }
}
