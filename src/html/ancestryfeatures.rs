use super::Template;
use crate::{
    data::{ancestry_features::AncestryFeature, traits::Translations, HasName},
    html::{render_trait_legend, render_traits, HtmlPage},
};
use std::borrow::Cow;

impl Template<&Translations> for AncestryFeature {
    fn render(&self, trait_descriptions: &Translations) -> Cow<'_, str> {
        let mut page = String::with_capacity(5000);
        page.push_str(&format!(
            "<h1><a href=\"/ancestryfeature/{}\">{}</a></h1>",
            self.url_name(),
            &self.name,
        ));
        render_traits(&mut page, &self.traits);
        page.push_str(&self.description);
        page.push_str("<hr/>");
        render_trait_legend(&mut page, &self.traits, trait_descriptions);
        Cow::Owned(page)
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Ancestry Feature")
    }

    fn render_index(elements: &[(Self, HtmlPage)]) -> String {
        let mut page = String::with_capacity(50_000);
        page.push_str("<div id=\"gridlist\">");
        for (ancestryfeature, _) in elements {
            page.push_str(&format!(
                "<span><a href=\"{}\">{}</a></span>",
                ancestryfeature.url_name(),
                ancestryfeature.name(),
            ));
        }
        page.push_str("</div>");
        page
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_eq_ignore_linebreaks, read_test_file, TRANSLATIONS};

    #[test]
    fn test_ancestry_feature_rendering() {
        let feature: AncestryFeature =
            serde_json::from_str(&read_test_file("ancestryfeatures.db/swim.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&feature.render(&TRANSLATIONS), include_str!("../../tests/html/azarketi_swim.html"));
    }
}
