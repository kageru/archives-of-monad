use super::Template;
use crate::{
    data::{class_features::ClassFeature, traits::Translations, HasName},
    html::{render_trait_legend, render_traits, HtmlPage},
};
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

lazy_static! {
    static ref PARENTHESIZED_EXPLANATION_REGEX: Regex = Regex::new(r" \(.+\)").unwrap();
}

impl Template<&Translations> for ClassFeature {
    fn render(&self, trait_descriptions: &Translations) -> Cow<'_, str> {
        let mut page = String::with_capacity(5000);
        page.push_str(&format!(
            "<h1><a href=\"/classfeature/{}\">{}</a> {}<span class=\"type\">Feature {}</span></h1><hr/>",
            self.url_name(),
            PARENTHESIZED_EXPLANATION_REGEX.replace(&self.name, ""),
            self.action_type.img(&self.number_of_actions),
            if self.level != 0 {
                self.level.to_string()
            } else {
                String::from("(Automatic)")
            },
        ));
        render_traits(&mut page, &self.traits);
        page.push_str(&self.description);
        page.push_str("<hr/>");
        render_trait_legend(&mut page, &self.traits, trait_descriptions);
        Cow::Owned(page)
    }

    fn render_index(elements: &[(Self, HtmlPage)]) -> String {
        let mut page = String::with_capacity(50_000);
        page.push_str("<div id=\"gridlist\">");
        for (classfeature, _) in elements {
            page.push_str(&format!(
                "<span><a href=\"{}\">{} {}</a></span>",
                classfeature.url_name(),
                classfeature.name(),
                classfeature.action_type.img(&classfeature.number_of_actions)
            ));
        }
        page.push_str("</div>");
        page
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Class Feature")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_eq_ignore_linebreaks, read_test_file, TRANSLATIONS};

    #[test]
    fn test_class_feature_rendering() {
        let feature: ClassFeature = serde_json::from_str(&read_test_file("classfeatures.db/evasion.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&feature.render(&TRANSLATIONS), include_str!("../../tests/html/evasion.html"));
    }
}
