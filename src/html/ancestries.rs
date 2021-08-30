use super::{inline_rarity_if_not_common, Template};
use crate::data::{ancestries::Ancestry, HasName};
use std::borrow::Cow;

impl Template<()> for Ancestry {
    fn render(&self, _: ()) -> Cow<'_, str> {
        Cow::Owned(format!("<h1>{}</h1><hr/>{}", &self.name(), &self.description))
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Ancestry")
    }

    fn render_index(elements: &[(Self, super::Page)]) -> String {
        let mut index = String::with_capacity(10_000);
        index.push_str("<h1>Ancestries</h1><hr/>");
        index.push_str("<div id=\"list\">");
        for (ancestry, _) in elements {
            index.push_str("<h2><a href=\"/ancestry/");
            index.push_str(&ancestry.url_name());
            index.push_str("\">");
            index.push_str(ancestry.name());
            index.push(' ');
            index.push_str(&inline_rarity_if_not_common(&ancestry.traits.rarity));
            index.push_str("</a></h2>");
        }
        index.push_str("</div>");
        index
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::read_test_file;

    use super::*;

    #[test]
    fn ancestry_rendering_test() {
        let spooder: Ancestry = serde_json::from_str(&read_test_file("ancestries.db/anadi.json")).expect("Deserialization failed");
        let spooder: String = spooder.render(()).lines().collect();
        let expected: String = include_str!("../../tests/html/spooder.html").lines().collect();
        assert_eq!(spooder, expected);
    }
}
