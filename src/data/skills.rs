use serde::{de, Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
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
}

impl<'de> Deserialize<'de> for Skill {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "acr" => Ok(Skill::Acrobatics),
            "arc" => Ok(Skill::Arcana),
            "ath" => Ok(Skill::Athletics),
            "cra" => Ok(Skill::Crafting),
            "dec" => Ok(Skill::Deception),
            "dip" => Ok(Skill::Diplomacy),
            "itm" => Ok(Skill::Intimidation),
            "med" => Ok(Skill::Medicine),
            "nat" => Ok(Skill::Nature),
            "occ" => Ok(Skill::Occultism),
            "prf" => Ok(Skill::Performance),
            "rel" => Ok(Skill::Religion),
            "soc" => Ok(Skill::Society),
            "ste" => Ok(Skill::Stealth),
            "sur" => Ok(Skill::Survival),
            "thi" => Ok(Skill::Thievery),
            s => Err(de::Error::invalid_value(de::Unexpected::Str(s), &"three letter acronym for skill")),
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
