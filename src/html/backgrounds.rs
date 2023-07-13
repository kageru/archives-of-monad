use crate::{
    data::{backgrounds::Background, HasName, ObjectName},
    html::{inline_rarity_if_not_common, render_traits, HtmlPage, Template},
};
use itertools::Itertools;
use std::fmt::Write;

impl Template<()> for Background {
    fn render(&self, _: ()) -> String {
        let mut page = String::with_capacity(1000);
        write!(
            page,
            "<h1><a href=\"/background/{}\">{}</a><span class=\"type\">Background</span></h1><hr/>",
            self.url_name(),
            self.name()
        );
        render_traits(&mut page, &self.traits);
        if !self.source.is_empty() {
            write!(page, "<b>Source </b>{}<br/>", &self.source);
            page.push_str("<hr/>");
        }
        page.push_str(&self.description);
        page.push_str("<hr/>");
        write!(page, "<b>Condensed:</b><br/>{}.", self.condensed());
        page
    }

    fn render_index(elements: &[(Self, HtmlPage)]) -> String {
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
            index.push_str(&bg.lore);
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

    fn category(&self) -> String {
        "Background".to_owned()
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
    fn test_background_template() {
        let field_medic: Background =
            serde_json::from_str(&read_test_file("backgrounds/field-medic.json")).expect("Deserialization of background failed");
        assert_eq_ignore_linebreaks(&field_medic.render(()), include_str!("../../tests/html/field_medic.html"));
    }

    #[test]
    fn test_background_template_haunted() {
        let haunted: Background =
            serde_json::from_str(&read_test_file("backgrounds/haunted.json")).expect("Deserialization of background failed");
        assert_eq_ignore_linebreaks(&haunted.render(()), include_str!("../../tests/html/haunted.html"));
    }

    #[test]
    fn test_background_index() {
        let field_medic: Background =
            serde_json::from_str(&read_test_file("backgrounds/field-medic.json")).expect("Deserialization of background failed");
        let haunted: Background =
            serde_json::from_str(&read_test_file("backgrounds/haunted.json")).expect("Deserialization of background failed");
        let bgs = vec![field_medic, haunted].into_iter().map(|bg| attach_html(bg, ())).collect_vec();
        assert_eq_ignore_linebreaks(
            &Template::render_index(&bgs),
            include_str!("../../tests/html/background_index.html"),
        );
    }
}
