use crate::{
    data::{ancestries::Ancestry, traits::Rarity, HasName},
    html::{render_traits, HtmlPage, Template},
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{borrow::Cow, fmt::Write};

lazy_static! {
    static ref CURSIVE_FLAVOUR_TEXT: Regex = Regex::new("<em>(.*?)</em>").unwrap();
}

impl Template<()> for Ancestry {
    fn render(&self, _: ()) -> Cow<'_, str> {
        let mut page = String::with_capacity(10_000);
        write!(page, "<h1><a href=\"/ancestry/{}\">{}</a></h1><hr/>", self.url_name(), &self.name,);
        render_traits(&mut page, &self.traits);
        write!(page, "<b>Source </b>{}<br/>{}", self.source, &self.description,);
        add_ancestry_feat_link(&self.url_name(), self.name(), &mut page);
        Cow::Owned(page)
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Ancestry")
    }

    fn render_index(elements: &[(Self, HtmlPage)]) -> String {
        let mut index = String::with_capacity(10_000);
        index.push_str(HEADER);
        index.push_str("<div id=\"list\">");

        render_rarity(elements, Rarity::Common, &mut index);
        render_rarity(elements, Rarity::Uncommon, &mut index);
        render_rarity(elements, Rarity::Rare, &mut index);

        index.push_str("</div>");
        index
    }

    fn header(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(HEADER))
    }
}

const HEADER: &str = r#"<div class="header">
<span><a href="/ancestry"><div>Ancestries</div></a></span>
<span><a href="/heritage" class="hoverlink"><div>Versatile Heritages</div></a></span>
</div>"#;

fn render_rarity(elements: &[(Ancestry, HtmlPage)], rarity: Rarity, page: &mut String) {
    let elements: Vec<_> = elements.iter().map(|(a, _)| a).filter(|a| a.traits.rarity == rarity).collect();
    if !elements.is_empty() {
        write!(page, "<div class=\"category rarity-{}\">", rarity.as_str().to_lowercase());
        page.push_str("<h1 class=\"category-title\">");
        write!(page, "{} Ancestries", rarity.as_str());
        page.push_str("</h1>");
        page.push_str("</div>");

        for ancestry in elements.iter() {
            write!(
                page,
                "<h1><a href=\"/ancestry/{}\">{}</a></h1><hr/>",
                ancestry.url_name(),
                ancestry.name()
            );
            let flavour_text_capture = CURSIVE_FLAVOUR_TEXT.captures(&ancestry.description);
            if let Some(m) = flavour_text_capture {
                page.push_str("<p>");
                page.push_str(&m[1]);
                page.push_str("</p>");
            }
        }
    }
}

pub fn add_ancestry_feat_link(url_name: &str, name: &str, page: &mut String) {
    write!(
        page,
        "<h2>Ancestry Feats</h2><p><a href=\"/feat/{}_index\">Click here for a list of all {} ancestry feats</a></p>",
        url_name, name
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_eq_ignore_linebreaks, read_test_file};

    #[test]
    fn ancestry_rendering_test() {
        let spooder: Ancestry = serde_json::from_str(&read_test_file("ancestries.db/anadi.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&spooder.render(()), include_str!("../../tests/html/spooder.html"));
    }
}
