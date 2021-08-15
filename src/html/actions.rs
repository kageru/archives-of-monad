use super::Template;
use crate::data::actions::Action;
use crate::data::HasName;
use crate::get_data_path;
use crate::html::render_traits;
use itertools::Itertools;
use std::borrow::Cow;
use std::{fs, io, io::BufReader};

impl Template<()> for Action {
    fn render(&self, _: ()) -> Cow<'_, str> {
        let mut page = String::with_capacity(2000);
        page.push_str(&format!(
            "<h1>{} {}</h1><hr/>",
            &self.name,
            self.action_type.img(&self.number_of_actions)
        ));
        let mut page = render_traits(page, &self.traits);
        page.push_str(&self.description);
        Cow::Owned(page)
    }
}

pub fn render_actions(folder: &str, target: &str) -> io::Result<()> {
    let mut all_actions = fs::read_dir(&format!("{}/packs/data/{}", get_data_path(), folder))?
        .filter_map(|f| {
            let filename = f.ok()?.path();
            let f = fs::File::open(&filename).ok()?;
            let reader = BufReader::new(f);
            let action: Action = serde_json::from_reader(reader).expect("Deserialization failed");
            Some(action)
        })
        .collect_vec();
    all_actions.sort_by_key(|s| s.name.clone());
    for action in &all_actions {
        fs::write(format!("{}/{}", target, action.url_name()), action.render(()).as_bytes())?;
    }
    fs::write(format!("{}/index.html", target), render_action_list(&all_actions))
}
fn render_action_list(all_actions: &[Action]) -> String {
    let mut page = String::with_capacity(10_000);
    page.push_str("<div id=\"gridlist\">");
    for action in all_actions {
        page.push_str(&format!(
            "<span><a href=\"{}\">{} {}</a></span>",
            action.url_name(),
            action.name(),
            action.action_type.img(&action.number_of_actions)
        ));
    }
    page.push_str("</div>");
    page
}

#[cfg(test)]
mod tests {
    use crate::tests::read_test_file;

    use super::*;

    #[test]
    fn test_action_template() {
        let aid: Action = serde_json::from_str(&read_test_file("actions.db/aid.json")).expect("Deserialization failed");
        let expected = include_str!("../../tests/html/aid.html");
        assert_eq!(aid.render(()).lines().collect::<String>(), expected.lines().collect::<String>());
    }
}
