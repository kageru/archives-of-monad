use super::{inline_rarity_if_not_common, render_traits, Template};
use crate::data::{backgrounds::Background, HasName};
use crate::html::Page;
use std::borrow::Cow;

impl Template<()> for Background {
    fn render(&self, _: ()) -> Cow<'_, str> {
        let mut page = String::with_capacity(1000);
        page.push_str("<h1>");
        page.push_str("<a href=\"/background/");
        page.push_str(&self.url_name());
        page.push_str("\">");
        page.push_str(&self.name);
        page.push_str("</a><span class=\"type\">");
        page.push_str("Background</span></h1><hr/>");
        render_traits(&mut page, &self.traits);
        page.push_str(&self.description);
        page.push_str("<b>Condensed:</b><br/>");
        page.push_str(&self.condensed());
        page.push('.');
        Cow::Owned(page)
    }

    fn render_index(elements: &[(Self, Page)]) -> String {
        let mut index = String::with_capacity(10_000);
        index.push_str("<h1>Backgrounds</h1><hr/>");
        index.push_str("<div id=\"list\">");
        for (bg, _) in elements {
            let condensed = bg.condensed();
            index.push_str("<p><h2><a href=\"");
            index.push_str(&bg.url_name());
            index.push_str("\">");
            index.push_str(&bg.name);
            index.push_str("</a> ");
            index.push_str(&inline_rarity_if_not_common(&bg.traits.rarity));
            index.push_str("</h2><hr/>");
            index.push_str(&condensed);
            index.push_str("</p>");
        }
        index.push_str("</div>");
        index
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Background")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{html::attach_page, tests::read_test_file};
    use itertools::Itertools;

    #[test]
    fn test_background_template() {
        let field_medic: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/field-medic.json")).expect("Deserialization of background failed");
        let expected = include_str!("../../tests/html/field_medic.html");
        assert_eq!(
            field_medic.render(()).lines().collect::<String>(),
            expected.lines().collect::<String>(),
        );
    }

    #[test]
    fn test_background_template_haunted() {
        let haunted: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/haunted.json")).expect("Deserialization of background failed");
        let expected = include_str!("../../tests/html/haunted.html");
        assert_eq!(haunted.render(()).lines().collect::<String>(), expected.lines().collect::<String>());
    }

    #[test]
    fn test_background_index() {
        let field_medic: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/field-medic.json")).expect("Deserialization of background failed");
        let haunted: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/haunted.json")).expect("Deserialization of background failed");
        let expected: String = include_str!("../../tests/html/background_index.html").lines().collect();
        let bgs = vec![field_medic, haunted].into_iter().map(|bg| attach_page(bg, ())).collect_vec();
        assert_eq!(Template::render_index(&bgs), expected);
    }
}
