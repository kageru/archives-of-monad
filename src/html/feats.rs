use itertools::Itertools;

use super::{Page, Template};
use crate::{
    data::{feats::Feat, traits::TraitDescriptions, HasName},
    html::{render_trait_legend, render_traits},
};
use std::{borrow::Cow, fs, io};

const CLASSES: &[&str] = &[
    "Alchemist",
    "Barbarian",
    "Bard",
    "Champion",
    "Cleric",
    "Druid",
    "Fighter",
    "Investigator",
    // "Magus"
    "Monk",
    "Oracle",
    "Ranger",
    "Rogue",
    "Sorcerer",
    // "Summoner"
    "Swashbuckler",
    "Witch",
    "Wizard",
];

impl Template<&TraitDescriptions> for Feat {
    fn render(&self, trait_descriptions: &TraitDescriptions) -> Cow<'_, str> {
        let mut page = String::with_capacity(5000);
        page.push_str(&format!(
            "<h1><a href=\"/feat/{}\">{}</a> {}<span class=\"type\">Feat {}</span></h1><hr/>",
            self.url_name(),
            &self.name,
            self.action_type.img(&self.actions),
            if self.level != 0 {
                self.level.to_string()
            } else {
                String::from("(Automatic)")
            },
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

    fn render_index(elements: &[(Self, Page)]) -> String {
        let feats = elements.iter().filter(|(f, _)| f.level != 0).collect_vec();
        render_feat_list(&feats, None)
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Feat")
    }

    fn render_subindices(target: &str, elements: &[(Self, Page)]) -> io::Result<()> {
        let feats = elements.iter().filter(|(f, _)| f.level != 0).collect_vec();
        for class in CLASSES {
            fs::write(
                &format!("{}/{}_index", target, class.to_lowercase()),
                render_feat_list(&feats, Some(class)),
            )?
        }
        Ok(())
    }
}

fn render_feat_list(feats: &[&(Feat, Page)], class: Option<&str>) -> String {
    let mut page = render_feat_list_header(&class);
    let class_trait = class.map(|c| c.to_lowercase());
    page.push_str("<table class=\"overview\">");
    page.push_str("<thead><tr><td>Name</td><td>Level</td></tr></thead>");
    for (feat, _) in feats.iter().filter(|(f, _)| match &class_trait {
        Some(t) => f.traits.value.contains(t),
        None => true,
    }) {
        page.push_str(&format!(
            "<tr><td><a href=\"{}\">{} {}</a><td>{}</td></tr>",
            feat.url_name(),
            feat.name,
            feat.action_type.img(&feat.actions),
            feat.level,
        ));
    }
    page.push_str("</table>");
    page
}

fn render_feat_list_header(class: &Option<&str>) -> String {
    let mut page = String::with_capacity(50_000);
    page.push_str(r#"<div class="header">"#);
    page.push_str(r#"<span><a href="index.html"><div>All</div></a></span>"#);
    for c in CLASSES {
        page.push_str(&format!(
            r#"<span><a href="{}_index"><div>{}</div></a></span>"#,
            c.to_lowercase(),
            c
        ));
    }
    page.push_str("</div>");
    match class {
        Some(c) => {
            page.push_str("<h1>");
            page.push_str(c);
            page.push_str(" Feats</h1><hr/>");
        }
        None => page.push_str("<h1>Feats</h1><hr/>"),
    }
    page.push_str("<div id=\"gridlist\">");
    page
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
