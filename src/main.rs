use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::{fmt, fs, io};

fn main() {
    let conditions: Vec<_> =
        fs::read_dir("/home/kageru/build/foundry-vtt---pathfinder-2e/packs/data/conditionitems.db")
            .expect("Could not read conditions")
            .map(|f| {
                let f = f.expect("bad path");
                println!("Reading {:?}", f);
                get_condition(f.path().to_str().expect("bad unicode"))
                    .expect(&format!("error during read/deser for {:?}", &f))
            })
            .collect();
    for condition in &conditions {
        println!("{}\n\n", condition);
    }
}

fn get_condition(filename: &str) -> io::Result<Condition> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    Ok(Condition::from(
        serde_json::from_reader::<_, JsonCondition>(reader)?,
    ))
}

#[derive(Deserialize)]
struct JsonCondition {
    data: ConditionData,
    name: String,
}

#[derive(Deserialize)]
struct ConditionData {
    description: ConditionDescription,
}

#[derive(Deserialize)]
struct ConditionDescription {
    value: String,
}

#[derive(Debug)]
struct Condition {
    name: String,
    description: String,
}

impl From<JsonCondition> for Condition {
    fn from(jc: JsonCondition) -> Self {
        Condition {
            name: jc.name,
            description: jc.data.description.value,
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}
