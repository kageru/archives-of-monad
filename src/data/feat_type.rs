use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FeatType {
    Ancestry,
    AncestryFeature,
    Heritage,
    Class,
    ClassFeature,
    Archetype,
    #[serde(rename = "deityboon")]
    Boon,
    Curse,
    Bonus,
    General,
    Skill,
}
