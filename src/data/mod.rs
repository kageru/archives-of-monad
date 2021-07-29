use serde::Deserialize;

pub mod ability_scores;
pub mod action_type;
pub mod actions;
pub mod ancestries;
pub mod ancestry_features;
pub mod archetypes;
pub mod class_features;
pub mod conditions;
pub mod damage;
pub mod deities;
pub mod feat_type;
pub mod size;
pub mod spells;
pub mod traits;

#[derive(Deserialize, Debug, PartialEq, Default)]
pub struct ValueWrapper<T> {
    value: T,
}

impl<T> From<T> for ValueWrapper<T> {
    fn from(t: T) -> Self {
        Self { value: t }
    }
}

#[macro_export]
macro_rules! impl_deser {
    ($type:ty :
    $($s:literal => $e:expr),+,
    expects: $expected:literal
    ) => {
        impl<'de> Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                match String::deserialize(deserializer)?.as_str() {
                    $($s => Ok($e)),+,
                    s => Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(s),
                        &$expected,
                    )),
                }
            }
        }
    }
}
