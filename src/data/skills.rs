use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, PartialEq, Display, Clone, Copy, Eq)]
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

impl TryFrom<&str> for Skill {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Acrobatics" => Ok(Skill::Acrobatics),
            "Arcana" => Ok(Skill::Arcana),
            "Athletics" => Ok(Skill::Athletics),
            "Crafting" => Ok(Skill::Crafting),
            "Deception" => Ok(Skill::Deception),
            "Diplomacy" => Ok(Skill::Diplomacy),
            "Intimidation" => Ok(Skill::Intimidation),
            "Medicine" => Ok(Skill::Medicine),
            "Nature" => Ok(Skill::Nature),
            "Occultism" => Ok(Skill::Occultism),
            "Performance" => Ok(Skill::Performance),
            "Religion" => Ok(Skill::Religion),
            "Society" => Ok(Skill::Society),
            "Stealth" => Ok(Skill::Stealth),
            "Survival" => Ok(Skill::Survival),
            "Thievery" => Ok(Skill::Thievery),
            _ => Err(()),
        }
    }
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
