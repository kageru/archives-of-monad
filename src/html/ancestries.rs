use super::Template;
use crate::data::{ancestries::Ancestry, traits::Rarity, HasName};
use crate::html::HtmlPage;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

lazy_static! {
    static ref CURSIVE_FLAVOUR_TEXT: Regex = Regex::new("<em>(.*?)</em>").unwrap();
}

impl Template<()> for Ancestry {
    fn render(&self, _: ()) -> Cow<'_, str> {
        Cow::Owned(format!(
            "<h1>{}</h1><hr/><b>Source </b>{}<br/>{}",
            &self.name(),
            &self.source,
            &self.description
        ))
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Ancestry")
    }

    fn render_index(elements: &[(Self, HtmlPage)]) -> String {
        let mut index = String::with_capacity(10_000);
        add_subheader(&mut index);
        index.push_str("<div id=\"list\">");

        render_rarity(&elements, Rarity::Common, &mut index);
        render_rarity(&elements, Rarity::Uncommon, &mut index);
        render_rarity(&elements, Rarity::Rare, &mut index);

        index.push_str("</div>");
        index
    }
}

fn add_subheader(page: &mut String) {
    page.push_str(r#"<div class="header">"#);
    page.push_str(r#"<span><a href="/ancestry" class="selected-header"><div>Ancestries</div></a></span>"#);
    page.push_str(r#"<span><a href="/heritage"><div>Versatile Heritages</div></a></span>"#);
    page.push_str("</div>");
}

fn render_rarity(elements: &[(Ancestry, HtmlPage)], rarity: Rarity, page: &mut String) {
    if elements.iter().any(|(a, _)| a.traits.rarity == rarity) {
        page.push_str(format!("<div class=\"category rarity-{}\">", rarity.as_str().to_lowercase()).as_str());
        page.push_str("<h1 class=\"category-title\">");
        page.push_str(format!("{} Ancestries", rarity.as_str()).as_str());
        page.push_str("</h1>");
        page.push_str("</div>");

        for (ancestry, _) in elements.iter().filter(|(a, _)| a.traits.rarity == rarity) {
            page.push_str("<h2 class=\"entry\"><a href=\"/ancestry/");
            page.push_str(&ancestry.url_name());
            page.push_str("\">");
            page.push_str(ancestry.name());
            page.push_str("</a></h2>");
            let flavour_text_capture = CURSIVE_FLAVOUR_TEXT.captures(&ancestry.description);
            match flavour_text_capture {
                Some(m) => page.push_str(m.get(1).unwrap().as_str()),
                None => {}
            }
        }
    }
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
