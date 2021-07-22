use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct Archetype {
    content: String,
    name: String,
}

impl fmt::Display for Archetype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.content)
    }
}
