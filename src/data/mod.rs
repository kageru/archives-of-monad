use core::fmt;
use serde::{de, Deserialize, Deserializer};

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
pub mod damage;
pub mod deities;
pub mod feat_type;
pub mod feats;
pub mod proficiency;
pub mod size;
pub mod skills;
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

pub trait HasName {
    fn name(&self) -> &str;

    fn url_name(&self) -> String {
        self.name()
            .to_lowercase()
            .replace(' ', "_")
            .replace('\'', "")
            .replace('(', "_")
            .replace(')', "_")
            .replace('?', "")
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

#[derive(Debug, PartialEq)]
pub struct I32Wrapper(i32);

impl<'de> Deserialize<'de> for I32Wrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        string_or_i32(deserializer).map(I32Wrapper)
    }
}

pub fn string_or_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrI32();

    impl<'de> de::Visitor<'de> for StringOrI32 {
        type Value = i32;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or i32")
        }
        fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
            Ok(value as i32)
        }
        fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
            Ok(value as i32)
        }
        fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
            Ok(value.to_owned().parse().unwrap_or(0))
        }
    }

    deserializer.deserialize_any(StringOrI32())
}

#[derive(Debug, PartialEq)]
pub struct StringWrapper(String);

impl<'de> Deserialize<'de> for StringWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_value_or_wrapper(deserializer)
    }
}

pub fn deserialize_value_or_wrapper<'de, D>(deserializer: D) -> Result<StringWrapper, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrWrapperVisitor();

    impl<'de> de::Visitor<'de> for StringOrWrapperVisitor {
        type Value = StringWrapper;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("String or ValueWrapper<String>")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(StringWrapper(v.to_owned()))
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>,
        {
            Ok(StringWrapper(map.next_entry::<String, String>()?.expect("no value here?").1))
        }
    }

    deserializer.deserialize_any(StringOrWrapperVisitor())
}
