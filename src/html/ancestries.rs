use super::Template;
use crate::data::ability_scores::AbilityBoost;
use crate::data::ancestries::AncestryItem;
use crate::data::ancestry_features::AncestryFeature;
use crate::data::size::Size;
use crate::data::vision::Vision;
use crate::data::{ancestries::Ancestry, traits::Rarity, HasName};
use crate::html::{render_traits, HtmlPage};
use convert_case::{Case, Casing};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;

lazy_static! {
    static ref CURSIVE_FLAVOUR_TEXT: Regex = Regex::new("<em>(.*?)</em>").unwrap();
}

impl Template<&[(AncestryFeature, HtmlPage)]> for Ancestry {
    fn render(&self, features: &[(AncestryFeature, HtmlPage)]) -> Cow<'_, str> {
        let mut page = String::with_capacity(10_000);
        page.push_str(&format!(
            "<h1><a href=\"/ancestry/{}\">{}</a></h1><hr/>",
            self.url_name(),
            &self.name,
        ));
        render_traits(&mut page, &self.traits);
        page.push_str(&format!("<b>Source </b>{}<br/>{}", self.source, &self.description,));
        add_hp(self.hp, &mut page);
        add_size(self.size, &mut page);
        add_speed(self.speed, &mut page);
        add_boosts(&self.boosts, &mut page);
        add_flaws(&self.flaws, &mut page);
        add_languages(
            &self.languages,
            self.num_of_additional_languages,
            &self.additional_languages,
            &mut page,
        );
        add_vision(&self.vision, features, &mut page);
        add_features(&self.ancestry_features, features, &mut page);
        page.push_str(&format!("<p><a href=\"/feat/{}_index\">Ancestry feats</a></p>", &self.url_name()));
        Cow::Owned(page)
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Ancestry")
    }

    fn render_index(elements: &[(Self, HtmlPage)]) -> String {
        let mut index = String::with_capacity(10_000);
        add_index_subheader(&mut index);
        index.push_str("<div id=\"list\">");

        render_rarity(elements, Rarity::Common, &mut index);
        render_rarity(elements, Rarity::Uncommon, &mut index);
        render_rarity(elements, Rarity::Rare, &mut index);

        index.push_str("</div>");
        index
    }
}

fn add_index_subheader(page: &mut String) {
    page.push_str(r#"<div class="header">"#);
    page.push_str(r#"<span><a href="/ancestry"><div>Ancestries</div></a></span>"#);
    page.push_str(r#"<span><a href="/heritage" class="hoverlink"><div>Versatile Heritages</div></a></span>"#);
    page.push_str("</div>");
}

fn render_rarity(elements: &[(Ancestry, HtmlPage)], rarity: Rarity, page: &mut String) {
    let elements: Vec<_> = elements.iter().map(|(a, _)| a).filter(|a| a.traits.rarity == rarity).collect();
    if !elements.is_empty() {
        page.push_str(&format!("<div class=\"category rarity-{}\">", rarity.as_str().to_lowercase()));
        page.push_str("<h1 class=\"category-title\">");
        page.push_str(&format!("{} Ancestries", rarity.as_str()));
        page.push_str("</h1>");
        page.push_str("</div>");

        for ancestry in elements.iter() {
            page.push_str("<h2 class=\"entry\"><a href=\"/ancestry/");
            page.push_str(&ancestry.url_name());
            page.push_str("\">");
            page.push_str(ancestry.name());
            page.push_str("</a></h2>");
            let flavour_text_capture = CURSIVE_FLAVOUR_TEXT.captures(&ancestry.description);
            if let Some(m) = flavour_text_capture {
                page.push_str("<p>");
                page.push_str(m.get(1).unwrap().as_str());
                page.push_str("</p>");
            }
        }
    }
}

fn add_hp(hp: i32, page: &mut String) {
    page.push_str("<h2>Hit Points</h2>");
    page.push_str(&format!("<p>{}</p>", hp));
}

fn add_size(size: Size, page: &mut String) {
    page.push_str("<h2>Size</h2>");
    page.push_str(&format!("<p>{}</p>", size.as_ref()));
}

fn add_speed(speed: i32, page: &mut String) {
    page.push_str("<h2>Speed</h2>");
    page.push_str(&format!("<p>{} feet</p>", speed));
}

fn add_boosts(boosts: &[AbilityBoost], page: &mut String) {
    if boosts.iter().any(|b| b.name().is_some()) {
        page.push_str("<h2>Ability Boosts</h2>");
        for boost in boosts.iter().flat_map(|b| b.name()) {
            page.push_str(&format!("<p>{}</p>", boost));
        }
    }
}

fn add_flaws(flaws: &[AbilityBoost], page: &mut String) {
    if flaws.iter().any(|b| b.name().is_some()) {
        page.push_str("<h2>Ability Flaws</h2>");
        for flaw in flaws.iter().flat_map(|f| f.name()) {
            page.push_str(&format!("<p>{}</p>", flaw));
        }
    }
}

fn add_languages(languages: &[String], num_of_additional_langs: i32, additional_langs: &[String], page: &mut String) {
    page.push_str("<h2>Languages</h2>");
    if !languages.is_empty() {
        for lang in languages {
            page.push_str(&format!("<p>{}</p>", lang.to_case(Case::Title)));
        }
    }
    if !additional_langs.is_empty() {
        page.push_str("<p>Additional languages equal to ");
        if num_of_additional_langs != 0 {
            page.push_str(&format!("{} + ", num_of_additional_langs));
        }
        page.push_str(&format!(
            "your Intelligence modifier (if it's positive). Choose from {}",
            additional_langs.iter().map(|l| l.to_case(Case::Title)).join(", ")
        ));
        page.push_str(", and any other languages to which you have access (such as the languages prevalent in your region).</p>");
    }
}

fn add_vision(vision: &Vision, all_features: &[(AncestryFeature, HtmlPage)], page: &mut String) {
    if !vision.is_normal() {
        let description = all_features
            .iter()
            .find_map(|(f, _)| (f.name == vision.name()).then(|| &f.description))
            .unwrap();
        page.push_str(&format!("<h2>{}</h2>", vision.name()));
        page.push_str(description);
    }
}

fn add_features(features: &[AncestryItem], all_features: &[(AncestryFeature, HtmlPage)], page: &mut String) {
    let features_by_name: HashMap<_, _> = all_features.iter().map(|(f, _)| (f.name(), f)).collect();
    for f in features {
        page.push_str(&format!("<h2>{}</h2>", f.name));
        let feature = *features_by_name
            .get(f.name.trim_start_matches("(Choice) "))
            .unwrap_or_else(|| panic!("Ancestryfeature {} not found", &f.name));
        page.push_str(&feature.description);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_eq_ignore_linebreaks, read_test_file};

    #[test]
    fn ancestry_rendering_test() {
        let spooder: Ancestry = serde_json::from_str(&read_test_file("ancestries.db/anadi.json")).expect("Deserialization failed");
        let fangs = serde_json::from_str(&read_test_file("ancestryfeatures.db/fangs.json")).expect("Deserialization failed");
        let change_shape =
            serde_json::from_str(&read_test_file("ancestryfeatures.db/change-shape-anadi.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(
            &spooder.render(&[(fangs, Default::default()), (change_shape, Default::default())]),
            include_str!("../../tests/html/spooder.html"),
        );
    }
}
