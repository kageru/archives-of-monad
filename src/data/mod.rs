use serde::Deserialize;

pub mod ability_scores;
pub mod action_type;
pub mod actions;
pub mod ancestries;
pub mod ancestry_features;
pub mod archetypes;
pub mod class_features;
pub mod conditions;
pub mod deities;
pub mod feat_type;
pub mod size;
pub mod traits;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ValueWrapper<T> {
    value: T,
}

impl<T> From<T> for ValueWrapper<T> {
    fn from(t: T) -> Self {
        Self { value: t }
    }
}
