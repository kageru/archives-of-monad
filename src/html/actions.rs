use super::Template;
use crate::data::actions::Action;
use crate::data::HasName;
use crate::html::{render_traits, Page};
use std::borrow::Cow;

impl Template<()> for Action {
    fn render(&self, _: ()) -> Cow<'_, str> {
        let mut page = String::with_capacity(2000);
        page.push_str(&format!(
            "<h1><a href=\"/action/{}\">{}</a> {}</h1><hr/>",
            &self.url_name(),
            &self.name,
            self.action_type.img(&self.number_of_actions)
        ));
        render_traits(&mut page, &self.traits);
        page.push_str(&self.description);
        Cow::Owned(page)
    }

    fn render_index(elements: &[(Self, Page)]) -> String {
        let mut page = String::with_capacity(10_000);
        page.push_str("<div id=\"gridlist\">");
        for (action, _) in elements {
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

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Action")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::attach_page;
    use crate::tests::{assert_eq_ignore_linebreaks, read_test_file};
    use itertools::Itertools;

    #[test]
    fn test_action_template() {
        let aid: Action = serde_json::from_str(&read_test_file("actions.db/aid.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&aid.render(()), include_str!("../../tests/html/aid.html"));
    }

    #[test]
    fn test_action_index() {
        let aid: Action = serde_json::from_str(&read_test_file("actions.db/aid.json")).expect("Deserialization failed");
        let boarding_assault: Action =
            serde_json::from_str(&read_test_file("actions.db/boarding-assault.json")).expect("Deserialization failed");
        let actions = vec![aid, boarding_assault].into_iter().map(|a| attach_page(a, ())).collect_vec();
        assert_eq_ignore_linebreaks(
            &Template::render_index(&actions),
            include_str!("../../tests/html/action_index.html"),
        );
    }
}
