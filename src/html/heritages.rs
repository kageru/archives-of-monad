use super::ancestries::add_ancestry_feat_link;
use super::Template;
use crate::data::traits::Rarity;
use crate::data::{heritages::Heritage, HasName};
use crate::html::HtmlPage;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

lazy_static! {
    static ref CURSIVE_FLAVOUR_TEXT: Regex = Regex::new("<em>(.*?)</em>").unwrap();
}

impl Template<()> for Heritage {
    fn render(&self, _: ()) -> Cow<'_, str> {
        let mut page = format!(
            "<h1><a href=\"/heritage/{}\">{}</a></h1><hr/><b>Source </b>{}<br/>{}",
            self.url_name(),
            &self.name(),
            &self.source,
            &self.description
        );
        add_ancestry_feat_link(&self.url_name(), self.name(), &mut page);
        Cow::Owned(page)
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Heritage")
    }

    fn render_index(elements: &[(Self, super::HtmlPage)]) -> String {
        let mut index = String::with_capacity(10_000);

        index.push_str(HEADER);
        index.push_str("<div id=\"list\">");

        let filtered: Vec<_> = elements.iter().filter(|(e, _)| e.ancestry.is_none()).collect();
        render_rarity(&filtered, Rarity::Common, &mut index);
        render_rarity(&filtered, Rarity::Uncommon, &mut index);
        render_rarity(&filtered, Rarity::Rare, &mut index);
        index.push_str("</div>");
        index
    }

    fn header(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(HEADER))
    }
}

const HEADER: &str = r#"<div class="header">
<span><a href="/ancestry" class="hoverlink"><div>Ancestries</div></a></span>
<span><a href="/heritage"><div>Versatile Heritages</div></a></span>
</div>"#;

fn render_rarity(elements: &[&(Heritage, HtmlPage)], rarity: Rarity, page: &mut String) {
    let elements: Vec<_> = elements.iter().filter(|(a, _)| a.traits.rarity == rarity).collect();
    if !elements.is_empty() {
        page.push_str(&format!("<div class=\"category rarity-{}\">", rarity.as_str().to_lowercase()));
        page.push_str(&format!("<h1 class=\"category-title\">{} Heritages</h1></div>", rarity.as_str()));
        for (_, p) in elements {
            page.push_str(&p.content);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_eq_ignore_linebreaks, read_test_file};

    #[test]
    fn ancestry_rendering_test() {
        let spooder: Heritage = serde_json::from_str(&read_test_file("heritages.db/aasimar.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&spooder.render(()), include_str!("../../tests/html/aasimar.html"));
    }
}
