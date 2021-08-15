use crate::data::traits::{Rarity, TraitDescriptions, Traits};
use convert_case::{Case, Casing};
use std::borrow::Cow;

pub mod actions;
pub mod conditions;
pub mod deities;
pub mod feats;
pub mod spells;

trait Template<AdditionalData> {
    fn render(&self, d: AdditionalData) -> Cow<'_, str>;
}

// No format!() here because there are called often, so the performance might actually matter
pub fn render_traits(mut page: String, traits: &Traits) -> String {
    page.push_str("<div class=\"traits\">");
    match traits.rarity {
        Some(Rarity::Common) => (),
        Some(r) => {
            let rarity = r.to_string();
            page.push_str("<span class=\"trait rarity-");
            page.push_str(&rarity.to_lowercase());
            page.push_str("\">");
            page.push_str(&rarity);
            page.push_str("</span>");
        }
        None => (),
    }
    for t in &traits.value {
        page.push_str("<span class=\"trait\">");
        page.push_str(&t.to_case(Case::Pascal));
        page.push_str("</span>");
    }
    page.push_str("</div>");
    page
}

pub fn render_trait_legend(mut page: String, traits: &Traits, trait_descriptions: &TraitDescriptions) -> String {
    page.push_str("<h2>Traits</h2><div class=\"trait-legend\">");
    if let Some(r) = traits.rarity {
        let rarity = r.to_string();
        page.push_str("<b>");
        page.push_str(&rarity);
        page.push_str("</b><p>");
        page.push_str(&trait_descriptions.0[&rarity]);
        page.push_str("</p>");
    };
    let mut page = traits
        .value
        .iter()
        .map(|name| name.to_case(Case::Pascal))
        // The rarity is sometimes redundantly included in the traits. Filter it here.
        .filter(|name| !matches!(traits.rarity.map(|r| r.to_string()), Some(n) if &n == name))
        .filter_map(|name| trait_descriptions.0.get(&name).cloned().map(|s| (name, s)))
        .fold(page, |mut p, (name, description)| {
            p.push_str("<b>");
            p.push_str(&name);
            p.push_str("</b><p>");
            p.push_str(&description);
            p.push_str("</p>");
            p
        });
    page.push_str("</div>");
    page
}

#[cfg(test)]
mod tests {
    use crate::data::{archetypes::Archetype, backgrounds::Background, feats::Feat};
    use crate::html::feats::FeatTemplate;
    use crate::tests::read_test_file;
    use crate::tests::DESCRIPTIONS;
    use askama::Template;
    use itertools::Itertools;

    #[test]
    fn test_feat_template() {
        let feat: Feat = serde_json::from_str(&read_test_file("feats.db/sever-space.json")).expect("Deserialization failed");
        let feat = FeatTemplate::new(feat, &DESCRIPTIONS);
        let expected = include_str!("../../tests/html/sever_space.html");
        assert_eq!(feat.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_background_template() {
        let field_medic: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/field-medic.json")).expect("Deserialization of background failed");
        let expected = include_str!("../../tests/html/field_medic.html");
        assert_eq!(field_medic.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_background_template_haunted() {
        let haunted: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/haunted.json")).expect("Deserialization of background failed");
        let expected = include_str!("../../tests/html/haunted.html");
        assert_eq!(haunted.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_archetype_template() {
        let assassin: Archetype =
            serde_json::from_str(&read_test_file("archetypes.db/assassin.json")).expect("Deserialization of background failed");
        let expected = include_str!("../../tests/html/assassin.html");
        assert_eq!(assassin.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }
}
