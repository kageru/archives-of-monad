use crate::impl_deser;
use serde::{Deserialize, Deserializer};

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

impl_deser! {
    Skill :
    "acr" => Skill::Acrobatics,
    "arc" => Skill::Arcana,
    "ath" => Skill::Athletics,
    "cra" => Skill::Crafting,
    "dec" => Skill::Deception,
    "dip" => Skill::Diplomacy,
    "itm" => Skill::Intimidation,
    "med" => Skill::Medicine,
    "nat" => Skill::Nature,
    "occ" => Skill::Occultism,
    "prf" => Skill::Performance,
    "rel" => Skill::Religion,
    "soc" => Skill::Society,
    "ste" => Skill::Stealth,
    "sur" => Skill::Survival,
    "thi" => Skill::Thievery,
    expects: "three letter acronym for skill"
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
