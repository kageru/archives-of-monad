use super::{inline_rarity_if_not_common, Template};
use crate::data::{heritages::Heritage, HasName};
use crate::html::HtmlPage;
use std::borrow::Cow;

impl Template<()> for Heritage {
    fn render(&self, _: ()) -> Cow<'_, str> {
        Cow::Owned(format!(
            "<h1>{}</h1><hr/><b>Source </b>{}<br/>{}",
            &self.name(),
            &self.source,
            &self.description
        ))
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Heritage")
    }

    fn render_index(elements: &[(Self, super::HtmlPage)]) -> String {
        let mut index = String::with_capacity(10_000);
        add_subheader(&mut index);
        index.push_str("<h1>Heritage</h1><hr/>");
        index.push_str("<div id=\"list\">");
        for (heritage, _) in elements
            .iter()
            .filter(|(e, _)| e.ancestry.is_none())
            .collect::<Vec<&(Heritage, HtmlPage)>>()
        {
            index.push_str("<h2><a href=\"/heritage/");
            index.push_str(&heritage.url_name());
            index.push_str("\">");
            index.push_str(heritage.name());
            index.push(' ');
            index.push_str(&inline_rarity_if_not_common(&heritage.traits.rarity));
            index.push_str("</a></h2>");
        }
        index.push_str("</div>");
        index
    }
}

fn add_subheader(page: &mut String) {
    page.push_str(r#"<div class="header">"#);
    page.push_str(r#"<span><a href="index.html"><div>Ancestries</div></a></span>"#);
    page.push_str(r#"<span><a href="index.html"><div>Versatile Heritages</div></a></span>"#);
    page.push_str("</div>");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_eq_ignore_linebreaks, read_test_file};

    #[test]
    fn ancestry_rendering_test() {
        //let spooder: Ancestry = serde_json::from_str(&read_test_file("heritages.db/aasimar.json")).expect("Deserialization failed");
        //assert_eq_ignore_linebreaks(&spooder.render(()), include_str!("../../tests/html/assimar.html"));
    }
}
