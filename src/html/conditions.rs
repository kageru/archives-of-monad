use crate::{
    data::{conditions::Condition, HasName},
    html::{HtmlPage, Template},
};

impl Template<()> for Condition {
    fn render(&self, _: ()) -> String {
        format!(
            "<h1><a href=\"/condition/{}\">{}</a><span class=\"type\">Condition</span></h1><hr>{}",
            self.url_name(),
            self.name,
            self.description,
        )
    }

    fn category(&self) -> String {
        "Condition".to_owned()
    }

    fn render_index(elements: &[(Self, HtmlPage)]) -> String {
        let mut index = String::with_capacity(50_000);
        for (_, page) in elements {
            index.push_str(&page.content);
        }
        index
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{
        html::attach_html,
        tests::{assert_eq_ignore_linebreaks, read_scraped_file},
    };

    use super::*;

    #[test]
    fn test_condition_template() {
        let conditions: Vec<Condition> = serde_json::from_str(&read_scraped_file("conditions")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&conditions[0].render(()), include_str!("../../tests/html/blinded.html"));
    }

    #[test]
    fn test_condition_list() {
        let conditions: Vec<Condition> = serde_json::from_str(&read_scraped_file("conditions")).expect("Deserialization failed");
        let rendered_conditions = conditions[0..2].into_iter().map(|c| attach_html(c.to_owned(), ())).collect_vec();
        assert_eq_ignore_linebreaks(
            &Template::render_index(&rendered_conditions),
            include_str!("../../tests/html/condition_index.html"),
        );
    }
}
