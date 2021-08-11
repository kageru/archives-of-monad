use itertools::Itertools;

use super::Template;
use crate::data::{conditions::Condition, HasName};
use crate::get_data_path;
use std::fs;
use std::io::{self, BufReader};

impl Template for Condition {
    fn render(&self) -> String {
        format!(
            "<h1><a href=\"{}\">{}</a><span class=\"type\">Condition</span></h1><hr>{}",
            self.url_name(),
            self.name,
            self.description,
        )
    }
}

pub fn render_conditions(source: &str, target: &str) -> io::Result<()> {
    fs::create_dir_all(target)?;
    let mut all_conditions = fs::read_dir(&format!("{}/packs/data/{}", get_data_path(), source))?
        .filter_map(|f| {
            let filename = f.ok()?.path();
            let f = fs::File::open(&filename).ok()?;
            let reader = BufReader::new(f);
            let condition: Condition = serde_json::from_reader(reader).expect("Deserialization failed");
            Some(condition)
        })
        .collect_vec();
    all_conditions.sort_by_key(|s| s.name.clone());
    let mut index = String::with_capacity(50_000);
    for condition in &all_conditions {
        let rendered = condition.render();
        fs::write(format!("{}/{}", target, condition.url_name()), &rendered)?;
        index.push_str(&rendered);
    }
    fs::write(format!("{}/index.html", target), &index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::read_test_file;

    #[test]
    fn test_condition_template() {
        let blinded: Condition = serde_json::from_str(&read_test_file("conditionitems.db/blinded.json")).expect("Deserialization failed");
        let expected: String = include_str!("../../tests/html/blinded.html").lines().collect();
        assert_eq!(blinded.render(), expected);
    }
}
