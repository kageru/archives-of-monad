use crate::{
    data::{conditions::Condition, HasName},
    html::{HtmlPage, Template},
};
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

    fn render_index(elements: &[(Self, HtmlPage)]) -> String {
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
    use crate::{
        html::attach_html,
        tests::{assert_eq_ignore_linebreaks, read_test_file},
    };
    use itertools::Itertools;

    #[test]
    fn test_condition_template() {
        let blinded: Condition = serde_json::from_str(&read_test_file("conditionitems.db/blinded.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&blinded.render(()), include_str!("../../tests/html/blinded.html"));
    }

    #[test]
    fn test_condition_list() {
        let blinded: Condition = serde_json::from_str(&read_test_file("conditionitems.db/blinded.json")).expect("Deserialization failed");
        let deafened: Condition = serde_json::from_str(&read_test_file("conditionitems.db/deafened.json")).expect("Deserialization failed");
        let conditions = vec![blinded, deafened].into_iter().map(|c| attach_html(c, ())).collect_vec();
        assert_eq_ignore_linebreaks(
            &Template::render_index(&conditions),
            include_str!("../../tests/html/condition_index.html"),
        );
    }
}
