use super::Template;
use crate::data::{deities::Deity, HasName};
use crate::html::Page;
use crate::HTML_FORMATTING_TAGS;
use std::borrow::Cow;

impl Template<()> for Deity {
    fn render(&self, _: ()) -> Cow<'_, str> {
        Cow::Borrowed(&self.content)
    }

    fn render_index(elements: &[(Self, Page)]) -> String {
        let mut index = String::with_capacity(10_000);
        index.push_str("<div id=\"gridlist\">");
        for (deity, _) in elements {
            index.push_str(&format!(
                "<span><a href=\"{}\">{}</a></span>",
                deity.url_name(),
                HTML_FORMATTING_TAGS.replace_all(
                    deity
                        .content
                        .split_once("</h1>")
                        .map(|(name, _)| name)
                        .unwrap_or_else(|| deity.name()),
                    ""
                )
            ));
        }
        index.push_str("</div>");
        index
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Deity")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        html::attach_page,
        tests::{assert_eq_ignore_linebreaks, read_test_file},
    };
    use itertools::Itertools;

    #[test]
    fn test_deity_template() {
        let asmodeus: Deity = serde_json::from_str(&read_test_file("deities.db/asmodeus.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&asmodeus.render(()), include_str!("../../tests/html/asmodeus.html"));
    }

    #[test]
    fn test_deity_list() {
        let asmodeus: Deity = serde_json::from_str(&read_test_file("deities.db/asmodeus.json")).expect("Deserialization failed");
        let pharasma: Deity = serde_json::from_str(&read_test_file("deities.db/pharasma.json")).expect("Deserialization failed");
        let deities = vec![asmodeus, pharasma].into_iter().map(|s| attach_page(s, ())).collect_vec();
        assert_eq_ignore_linebreaks(&Template::render_index(&deities), include_str!("../../tests/html/deity_index.html"));
    }
}
