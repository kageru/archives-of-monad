use crate::{
    data::{class_features::ClassFeature, traits::Translations, HasName},
    html::{render_trait_legend, render_traits, HtmlPage, Template},
};
use std::fmt::Write;

impl Template<&Translations> for ClassFeature {
    fn render(&self, trait_descriptions: &Translations) -> String {
        let mut page = String::with_capacity(5000);
        write!(
            page,
            "<h1><a href=\"/classfeature/{}\">{}</a> {}<span class=\"type\">Feature {}</span></h1><hr/>",
            self.url_name(),
            &self.name,
            self.action_type.img(&self.number_of_actions),
            if self.level != 0 {
                self.level.to_string()
            } else {
                String::from("(Automatic)")
            },
        );
        render_traits(&mut page, &self.traits);
        page.push_str(&self.description);
        page.push_str("<hr/>");
        render_trait_legend(&mut page, &self.traits, trait_descriptions);
        page
    }

    fn category(&self) -> String {
        "Class Feature".to_owned()
    }

    fn render_index(elements: &[(Self, HtmlPage)]) -> String {
        let mut page = String::with_capacity(50_000);
        page.push_str("<div id=\"gridlist\">");
        for (classfeature, _) in elements {
            write!(
                page,
                "<span><a href=\"{}\">{} {}</a></span>",
                classfeature.url_name(),
                classfeature.name(),
                classfeature.action_type.img(&classfeature.number_of_actions)
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
    fn test_class_feature_rendering() {
        let feature: ClassFeature = serde_json::from_str(&read_test_file("classfeatures/evasion.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&feature.render(&TRANSLATIONS), include_str!("../../tests/html/evasion.html"));
    }
}
