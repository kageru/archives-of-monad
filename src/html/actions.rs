use super::Template;
use crate::data::actions::Action;
use crate::data::HasName;
use crate::html::render_traits;
use std::borrow::Cow;

impl Template<()> for Action {
    fn render(&self, _: ()) -> Cow<'_, str> {
        let mut page = String::with_capacity(2000);
        page.push_str(&format!(
            "<h1>{} {}</h1><hr/>",
            &self.name,
            self.action_type.img(&self.number_of_actions)
        ));
        render_traits(&mut page, &self.traits);
        page.push_str(&self.description);
        Cow::Owned(page)
    }

    fn render_index(elements: &[Self]) -> String {
        let mut page = String::with_capacity(10_000);
        page.push_str("<div id=\"gridlist\">");
        for action in elements {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::read_test_file;

    #[test]
    fn test_action_template() {
        let aid: Action = serde_json::from_str(&read_test_file("actions.db/aid.json")).expect("Deserialization failed");
        let expected = include_str!("../../tests/html/aid.html");
        assert_eq!(aid.render(()).lines().collect::<String>(), expected.lines().collect::<String>());
    }
}
