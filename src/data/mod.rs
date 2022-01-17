use self::{
    actions::Action,
    ancestries::Ancestry,
    archetypes::Archetype,
    backgrounds::Background,
    class_features::ClassFeature,
    classes::Class,
    conditions::Condition,
    creature::{Creature, Npc},
    deities::Deity,
    feats::Feat,
    heritages::Heritage,
    spells::Spell,
};
use crate::data::ancestry_features::AncestryFeature;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::cmp::Ordering;

pub mod ability_scores;
pub mod action_type;
pub mod actions;
pub mod ancestries;
pub mod ancestry_features;
pub mod archetypes;
pub mod backgrounds;
pub mod boons_and_curses;
pub mod class_features;
pub mod classes;
pub mod conditions;
pub mod creature;
pub mod damage;
pub mod deities;
pub mod equipment;
pub mod feat_type;
pub mod feats;
pub mod heritages;
pub mod proficiency;
pub mod size;
pub mod skills;
pub mod spells;
pub mod traits;
pub mod vision;

lazy_static! {
    static ref URL_REPLACE_CHARACTERS: Regex = Regex::new("[ -]+").unwrap();
    static ref URL_REMOVE_CHARACTERS: Regex = Regex::new("[^a-z0-9_]").unwrap();
}

#[derive(Deserialize, Debug, PartialEq, Default, Clone, Copy, Eq)]
pub struct ValueWrapper<T> {
    value: T,
}

impl<T> From<T> for ValueWrapper<T> {
    fn from(t: T) -> Self {
        Self { value: t }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectName<'a>(pub &'a str);

impl<'a> HasName for ObjectName<'a> {
    fn name(&self) -> &str {
        self.0
    }
}

pub trait HasName {
    fn name(&self) -> &str;

    fn url_name(&self) -> String {
        URL_REMOVE_CHARACTERS
            .replace_all(URL_REPLACE_CHARACTERS.replace_all(&self.name().to_lowercase(), "_").as_ref(), "")
            .to_string()
    }

    fn without_variant(&self) -> &str {
        self.name().split(" (").next().unwrap_or_else(|| self.name())
    }
}

pub trait HasLevel {
    fn level(&self) -> i32;
}

pub fn ensure_trailing_unit(speed: &str) -> String {
    let speed = speed.trim();
    if speed.ends_with(" feet") {
        speed.to_string()
    } else {
        format!("{} feet", speed)
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

macro_rules! ord_by_name {
    ($type:ty) => {
        impl PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.name.cmp(&other.name))
            }
        }
        impl Ord for $type {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.name.cmp(&other.name)
            }
        }

        impl HasName for $type {
            fn name(&self) -> &str {
                &self.name
            }
        }
    };
}

macro_rules! has_level {
    ($type:ty) => {
        impl HasLevel for $type {
            fn level(&self) -> i32 {
                self.level
            }
        }
    };
}
macro_rules! has_name {
    ($type:ty) => {
        impl HasName for $type {
            fn name(&self) -> &str {
                &self.name
            }
        }
    };
}

macro_rules! ord_by_name_and_level {
    ($type:ty) => {
        impl PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(&other))
            }
        }
        impl Ord for $type {
            fn cmp(&self, other: &Self) -> Ordering {
                match &self.level().cmp(&other.level()) {
                    Ordering::Equal => self.name().cmp(&other.name()),
                    &o => o,
                }
            }
        }
    };
}

ord_by_name!(Action);
ord_by_name!(Ancestry);
ord_by_name!(AncestryFeature);
ord_by_name!(Heritage);
ord_by_name!(Archetype);
ord_by_name!(Background);
ord_by_name!(Class);
ord_by_name!(ClassFeature);
ord_by_name!(Condition);
ord_by_name!(Deity);
ord_by_name_and_level!(Npc);
ord_by_name_and_level!(Feat);
has_name!(Feat);
has_level!(Creature);
has_name!(Creature);
ord_by_name_and_level!(Spell);

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Eq)]
    struct NamedWithLevel {
        level: i32,
        name: &'static str,
    }

    has_name!(NamedWithLevel);
    has_level!(NamedWithLevel);
    ord_by_name_and_level!(NamedWithLevel);

    #[test]
    fn level_name_ordering_test() {
        let lower = NamedWithLevel { name: "ZZZ", level: 1 };
        let higher = NamedWithLevel { name: "AAA", level: 10 };
        assert!(lower < higher);
        let lower = NamedWithLevel { name: "AAA", level: 1 };
        let higher = NamedWithLevel { name: "AAA", level: 10 };
        assert!(lower < higher);
        let lower = NamedWithLevel { name: "AAA", level: 10 };
        let higher = NamedWithLevel { name: "BBB", level: 10 };
        assert!(lower < higher);
    }
}
