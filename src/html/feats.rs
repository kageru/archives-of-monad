use super::Template;
use crate::{
    data::{feats::Feat, traits::TraitDescriptions, HasName},
    html::{render_trait_legend, render_traits},
};
use std::borrow::Cow;

impl Template<&TraitDescriptions> for Feat {
    fn render(&self, trait_descriptions: &TraitDescriptions) -> Cow<'_, str> {
        let mut page = String::with_capacity(5000);
        page.push_str(&format!(
            "<h1><a href=\"/feat/{}\">{}</a> {}<span class=\"type\">Feat {}</span></h1><hr/>",
            self.url_name(),
            &self.name,
            self.action_type.img(&self.actions),
            self.level
        ));
        render_traits(&mut page, &self.traits);
        if !self.prerequisites.is_empty() {
            page.push_str("<b>Prerequisites</b> ");
            page.push_str(&self.prerequisites.join(","));
            page.push_str("<hr/>");
        }
        page.push_str(&self.description);
        page.push_str("<hr/>");
        render_trait_legend(&mut page, &self.traits, trait_descriptions);
        Cow::Owned(page)
    }

    fn render_index(elements: &[Self]) -> String {
        let mut page = String::with_capacity(50_000);
        page.push_str("<div id=\"gridlist\">");
        for feat in elements {
            page.push_str(&format!(
                "<span><a href=\"{}\">{} {}</a></span>",
                feat.url_name(),
                feat.name(),
                feat.action_type.img(&feat.actions)
            ));
        }
        page.push_str("</div>");
        page
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Feat")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{read_test_file, DESCRIPTIONS};

    #[test]
    fn test_feat_template() {
        let feat: Feat = serde_json::from_str(&read_test_file("feats.db/sever-space.json")).expect("Deserialization failed");
        let expected = include_str!("../../tests/html/sever_space.html");
        assert_eq!(
            feat.render(&DESCRIPTIONS).lines().collect::<String>(),
            expected.lines().collect::<String>()
        );
    }
}
