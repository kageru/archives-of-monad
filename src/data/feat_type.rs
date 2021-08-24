use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Eq)]
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
