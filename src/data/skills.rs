use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, PartialEq, IntoStaticStr, AsRefStr, Clone, Eq, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum Skill {
    Acrobatics,
    Arcana,
    Athletics,
    Crafting,
    Deception,
    Diplomacy,
    Intimidation,
    Medicine,
    Nature,
    Occultism,
    Performance,
    Religion,
    Society,
    Stealth,
    Survival,
    Thievery,
    Lore(String),
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
        let json = r#"{ "value": ["occultism"] }"#;
        let skill: ValueWrapper<Vec<Skill>> = serde_json::from_str(json).unwrap();
        assert_eq!(skill.value[0], Skill::Occultism);
    }
}
