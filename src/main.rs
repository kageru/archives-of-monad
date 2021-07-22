use crate::data::actions::{Action, JsonAction};
use data::conditions::{Condition, JsonCondition};
use std::fs::File;
use std::io::BufReader;
use std::{fs, io};

mod data;

fn main() {
    let dir = std::env::args().nth(1).expect("No path given");
    let conditions: Vec<_> = fs::read_dir(format!("{}{}", &dir, "packs/data/conditionitems.db"))
        .expect("Could not read conditions")
        .map(|f| {
            let f = f.expect("bad path");
            println!("Reading {:?}", f);
            get_condition(f.path().to_str().expect("bad unicode")).expect(&format!("error during read/deser for {:?}", &f))
        })
        .collect();
    let actions: Vec<_> = fs::read_dir(format!("{}{}", &dir, "packs/data/actions.db"))
        .expect("Could not read conditions")
        .map(|f| {
            let f = f.expect("bad path");
            println!("Reading {:?}", f);
            get_action(f.path().to_str().expect("bad unicode")).expect(&format!("error during read/deser for {:?}", &f))
        })
        .collect();
    for condition in &conditions {
        println!("{}\n\n", condition);
    }
    for action in &actions {
        println!("{}\n\n", action);
    }
}

fn get_condition(filename: &str) -> io::Result<Condition> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    Ok(Condition::from(serde_json::from_reader::<_, JsonCondition>(reader)?))
}

fn get_action(filename: &str) -> io::Result<Action> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    Ok(Action::from(serde_json::from_reader::<_, JsonAction>(reader)?))
}
