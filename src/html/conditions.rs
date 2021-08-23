use super::Template;
use crate::data::{conditions::Condition, HasName};
use std::borrow::Cow;

impl Template<()> for Condition {
    fn render(&self, _: ()) -> Cow<'_, str> {
        Cow::Owned(format!(
            "<h1><a href=\"{}\">{}</a><span class=\"type\">Condition</span></h1><hr>{}",
            self.url_name(),
            self.name,
            self.description,
        ))
    }

    fn render_index(elements: &[Self]) -> String {
        let mut index = String::with_capacity(50_000);
        for condition in elements {
            index.push_str(&condition.render(()));
        }
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::read_test_file;

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
        assert_eq!(Template::render_index(&[blinded, deafened]), expected);
    }
}
