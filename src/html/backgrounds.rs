use super::{render_traits, Template};
use crate::data::backgrounds::Background;
use crate::data::traits::Rarity;
use crate::data::HasName;
use crate::html::render_traits_inline;
use std::borrow::Cow;

impl Template<()> for Background {
    fn render(&self, _: ()) -> Cow<'_, str> {
        let mut page = String::with_capacity(1000);
        page.push_str("<h1>");
        page.push_str(&self.name);
        page.push_str("<span class=\"type\">Background</span></h1><hr/>");
        render_traits(&mut page, &self.traits);
        page.push_str(&self.description);
        page.push_str("<b>Condensed:</b><br/>");
        page.push_str(&self.condensed());
        page.push('.');
        Cow::Owned(page)
    }

    fn render_index(elements: &[Self]) -> String {
        let mut index = String::with_capacity(10_000);
        index.push_str("<h1>Backgrounds</h1><hr/>");
        index.push_str("<div id=\"list\">");
        for bg in elements {
            let condensed = bg.condensed();
            index.push_str("<p><h2><a href=\"");
            index.push_str(&bg.url_name());
            index.push_str("\">");
            index.push_str(&bg.name);
            index.push_str("</a><span class=\"type\">");
            if bg.traits.rarity != Some(Rarity::Common) || !bg.traits.value.is_empty() {
                render_traits_inline(&mut index, &bg.traits);
            }
            index.push_str("</span></h2><hr/>");
            index.push_str(&condensed);
            index.push_str("</p>");
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
        assert_eq!(Template::render_index(&[field_medic, haunted]), expected);
    }
}
