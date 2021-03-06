use crate::{
    data::{ancestry_features::AncestryFeature, traits::Translations, HasName},
    html::{render_trait_legend, render_traits, HtmlPage, Template},
};
use std::{borrow::Cow, fmt::Write};

impl Template<&Translations> for AncestryFeature {
    fn render(&self, trait_descriptions: &Translations) -> Cow<'_, str> {
        let mut page = String::with_capacity(5000);
        write!(
            page,
            "<h1><a href=\"/ancestryfeature/{}\">{}</a><span class=\"type\">Ancestry Feature</span></h1><hr/>",
            self.url_name(),
            &self.name,
        );
        render_traits(&mut page, &self.traits);
        page.push_str("<hr/>");
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
            write!(
                page,
                "<span><a href=\"{}\">{}</a></span>",
                ancestryfeature.url_name(),
                ancestryfeature.name(),
            );
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
