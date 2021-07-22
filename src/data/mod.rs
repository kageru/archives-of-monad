use serde::Deserialize;

pub mod ability_scores;
pub mod actions;
pub mod ancestries;
pub mod ancestry_features;
pub mod archetypes;
pub mod conditions;
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
