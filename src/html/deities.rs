use super::Template;
use crate::data::{deities::Deity, HasName};
use crate::HTML_FORMATTING_TAGS;
use std::borrow::Cow;

impl Template<()> for Deity {
    fn render(&self, _: ()) -> Cow<'_, str> {
        Cow::Borrowed(&self.content)
    }

    fn render_index(elements: &[Self]) -> String {
        let mut index = String::with_capacity(10_000);
        index.push_str("<div id=\"gridlist\">");
        for deity in elements {
            index.push_str(&format!(
                "<span><a href=\"{}\">{}</a></span>",
                deity.url_name(),
                HTML_FORMATTING_TAGS.replace_all(deity.content.lines().next().unwrap_or_else(|| deity.name()), "")
            ));
        }
        index.push_str("</div>");
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::read_test_file;

    #[test]
    fn test_deity_template() {
        let asmodeus: Deity = serde_json::from_str(&read_test_file("deities.db/asmodeus.json")).expect("Deserialization failed");
        let expected = include_str!("../../tests/html/asmodeus.html");
        assert_eq!(
            asmodeus.render(()).lines().collect::<String>(),
            expected.lines().collect::<String>()
        );
    }

    #[test]
    fn test_deity_list() {
        let asmodeus: Deity = serde_json::from_str(&read_test_file("deities.db/asmodeus.json")).expect("Deserialization failed");
        let pharasma: Deity = serde_json::from_str(&read_test_file("deities.db/pharasma.json")).expect("Deserialization failed");
        let expected: String = include_str!("../../tests/html/deity_index.html").lines().collect();
        assert_eq!(Template::render_index(&[asmodeus, pharasma]), expected);
    }
}
