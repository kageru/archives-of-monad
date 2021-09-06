use super::Template;
use crate::data::{conditions::Condition, HasName};
use crate::html::Page;
use std::borrow::Cow;

impl Template<()> for Condition {
    fn render(&self, _: ()) -> Cow<'_, str> {
        Cow::Owned(format!(
            "<h1><a href=\"/condition/{}\">{}</a><span class=\"type\">Condition</span></h1><hr>{}",
            self.url_name(),
            self.name,
            self.description,
        ))
    }

    fn render_index(elements: &[(Self, Page)]) -> String {
        let mut index = String::with_capacity(50_000);
        for (_, page) in elements {
            index.push_str(&page.content);
        }
        index
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Condition")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{html::attach_page, tests::read_test_file};
    use itertools::Itertools;

    #[test]
    fn test_condition_template() {
        let blinded: Condition = serde_json::from_str(&read_test_file("conditionitems.db/blinded.json")).expect("Deserialization failed");
        let expected: String = include_str!("../../tests/html/blinded.html").lines().collect();
        assert_eq!(blinded.render(()), expected);
    }

    #[test]
    fn test_condition_list() {
        let blinded: Condition = serde_json::from_str(&read_test_file("conditionitems.db/blinded.json")).expect("Deserialization failed");
        let deafened: Condition = serde_json::from_str(&read_test_file("conditionitems.db/deafened.json")).expect("Deserialization failed");
        let expected: String = include_str!("../../tests/html/condition_index.html").lines().collect();
        let conditions = vec![blinded, deafened].into_iter().map(|c| attach_page(c, ())).collect_vec();
        assert_eq!(Template::render_index(&conditions), expected);
    }
}
