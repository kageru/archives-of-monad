use itertools::Itertools;

use super::{inline_rarity_if_not_common, render_traits, Template};
use crate::data::ObjectName;
use crate::data::{backgrounds::Background, HasName};
use crate::html::Page;
use std::borrow::Cow;

impl Template<()> for Background {
    fn render(&self, _: ()) -> Cow<'_, str> {
        let mut page = String::with_capacity(1000);
        page.push_str(&format!(
            "<h1><a href=\"/background/{}\">{}</a><span class=\"type\">Background</span></h1><hr/>",
            self.url_name(),
            self.name()
        ));
        render_traits(&mut page, &self.traits);
        if !self.source.is_empty() {
            page.push_str(&format!("<b>Source </b>{}<br/>", &self.source));
            page.push_str("<hr/>");
        }
        page.push_str(&self.description);
        page.push_str("<hr/>");
        page.push_str(&format!("<b>Condensed:</b><br/>{}.", self.condensed()));
        Cow::Owned(page)
    }

    fn render_index(elements: &[(Self, Page)]) -> String {
        let mut index = String::with_capacity(10_000);
        index.push_str("<h1>Backgrounds</h1><hr/>");
        index.push_str("<table class=\"overview\"><thead><tr><td>Name</td><td>Boost(s)</td><td>Lore</td><td>Feat</td></tr></thead>");
        for (bg, _) in elements {
            index.push_str("<tr><td><a href=\"/background/");
            index.push_str(&bg.url_name());
            index.push_str("\">");
            index.push_str(&bg.name);
            index.push_str("</a> ");
            index.push_str(&inline_rarity_if_not_common(&bg.traits.rarity));
            index.push_str("</td><td>");
            index.push_str(&bg.boosts.iter().join(", "));
            index.push_str("</td><td>");
            index.push_str(match bg.lore.as_str() {
                "Lore" => "varies",
                "" => "none",
                lore => lore,
            });
            index.push_str("</td><td>");
            index.push_str(
                &bg.feats
                    .first()
                    .map(|f| {
                        let featname = ObjectName(f);
                        format!("<a href=\"/feat/{}\">{}</a>", featname.url_name(), featname.without_variant())
                    })
                    .unwrap_or_else(|| String::from("none")),
            );
            index.push_str("</td></tr>");
        }
        index.push_str("</table>");
        index
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Background")
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
    fn test_background_template() {
        let field_medic: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/field-medic.json")).expect("Deserialization of background failed");
        assert_eq_ignore_linebreaks(&field_medic.render(()), include_str!("../../tests/html/field_medic.html"));
    }

    #[test]
    fn test_background_template_haunted() {
        let haunted: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/haunted.json")).expect("Deserialization of background failed");
        assert_eq_ignore_linebreaks(&haunted.render(()), include_str!("../../tests/html/haunted.html"));
    }

    #[test]
    fn test_background_index() {
        let field_medic: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/field-medic.json")).expect("Deserialization of background failed");
        let haunted: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/haunted.json")).expect("Deserialization of background failed");
        let bgs = vec![field_medic, haunted].into_iter().map(|bg| attach_page(bg, ())).collect_vec();
        assert_eq_ignore_linebreaks(
            &Template::render_index(&bgs),
            include_str!("../../tests/html/background_index.html"),
        );
    }
}
