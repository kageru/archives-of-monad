use super::render_traits;
use crate::data::spells::{Area, Spell, SpellCategory, SpellTradition};
use crate::data::traits::TraitDescriptions;
use crate::data::{HasLevel, HasName};
use crate::html::{render_trait_legend, render_traits_inline, Template};
use crate::html::{write_full_page, Page};
use crate::{get_action_img, HTML_FORMATTING_TAGS};
use itertools::Itertools;
use std::borrow::Cow;
use std::io;

impl Template<&TraitDescriptions> for Spell {
    fn render(&self, trait_descriptions: &TraitDescriptions) -> std::borrow::Cow<'_, str> {
        Cow::Owned(render_spell(self, trait_descriptions))
    }

    fn render_subindices(target: &str, elements: &[(Self, Page)]) -> io::Result<()> {
        write_full_page(
            &format!("{}/{}", target, "arcane"),
            "Arcane Spells",
            &render_tradition(elements, SpellTradition::Arcane),
        )?;
        write_full_page(
            &format!("{}/{}", target, "divine"),
            "Divine Spells",
            &render_tradition(elements, SpellTradition::Divine),
        )?;
        write_full_page(
            &format!("{}/{}", target, "occult"),
            "Occult Spells",
            &render_tradition(elements, SpellTradition::Occult),
        )?;
        write_full_page(
            &format!("{}/{}", target, "primal"),
            "Primal Spells",
            &render_tradition(elements, SpellTradition::Primal),
        )
    }

    fn render_index(elements: &[(Self, Page)]) -> String {
        render_full_spell_list(elements)
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed(if self.is_cantrip() { SpellCategory::Cantrip } else { self.category }.into())
    }
}

fn render_spell(spell: &Spell, trait_descriptions: &TraitDescriptions) -> String {
    let mut page = String::with_capacity(4000);
    page.push_str(&format!(
        r#"<h1><a href="/spell/{}">{}</a><span class="type">{} {}</span></h1><hr/>"#,
        spell.url_name(),
        spell.name(),
        spell.category(),
        spell.level,
    ));
    render_traits(&mut page, &spell.traits);
    if !spell.traditions.is_empty() {
        page.push_str("<b>Traditions</b> ");
        page.push_str(&spell.traditions.iter().map_into::<&str>().join(", "));
        page.push_str("<br/>");
    }
    page.push_str("<b>Cast</b> ");
    page.push_str(get_action_img(&spell.time).unwrap_or(&spell.time));
    page.push_str(spell.components.as_str());
    page.push_str("<br/>");
    if !spell.cost.is_empty() {
        page.push_str(&format!("<b>Cost</b> {}<br/>", &spell.cost));
    }
    if !spell.secondary_casters.is_empty() {
        page.push_str(&format!("<b>Secondary Casters</b> {}<br/>", &spell.secondary_casters));
    }
    if !spell.primary_check.is_empty() {
        page.push_str(&format!("<b>Primary Check</b> {}<br/>", &spell.primary_check));
    }
    if !spell.secondary_check.is_empty() {
        page.push_str(&format!("<b>Secondary Checks</b> {}<br/>", &spell.secondary_check));
    }
    match (&spell.area_string, spell.area) {
        (Some(area), _) => page.push_str(&format!("<b>Area</b> {}<br/>", area)),
        (None, area) if area != Area::None => page.push_str(&format!("<b>Area</b> {}<br/>", area)),
        _ => (),
    }
    if !spell.range.is_empty() {
        page.push_str(&format!("<b>Range</b> {}<br/>", &spell.range));
    }
    if !spell.target.is_empty() {
        page.push_str(&format!("<b>Target</b> {}<br/>", &spell.target));
    }
    if !spell.duration.is_empty() {
        page.push_str(&format!("<b>Duration</b> {}<br/>", &spell.duration));
    }
    if let Some(save) = spell.save {
        page.push_str("<b>Saving Throw</b> ");
        if spell.basic_save {
            page.push_str("basic ");
        }
        page.push_str(save.into());
        page.push_str("<br/>");
    }
    page.push_str("<hr/>");
    page.push_str(&spell.description);
    page.push_str("<hr/>");
    render_trait_legend(&mut page, &spell.traits, trait_descriptions);
    page
}

fn add_spell_header(mut page: String) -> String {
    page.push_str(r#"<div class="header">"#);
    page.push_str(r#"<span><a href="index.html"><div>All</div></a></span>"#);
    page.push_str(r#"<span><a href="arcane"><div>Arcane</div></a></span>"#);
    page.push_str(r#"<span><a href="divine"><div>Divine</div></a></span>"#);
    page.push_str(r#"<span><a href="occult"><div>Occult</div></a></span>"#);
    page.push_str(r#"<span><a href="primal"><div>Primal</div></a></span>"#);
    page.push_str("</div>");
    page
}

fn render_full_spell_list(spells: &[(Spell, Page)]) -> String {
    let mut page = String::with_capacity(100_000);
    page = add_spell_header(page);
    page.push_str("<h1>All Spells</h1><hr/><br/>");
    add_spell_list(&mut page, spells, |_| true);
    page
}

fn render_tradition(spells: &[(Spell, Page)], tradition: SpellTradition) -> String {
    let mut page = String::with_capacity(50_000);
    page = add_spell_header(page);
    page.push_str(&format!("<h1>{} Spell List</h1><hr/><br/><div id=\"list\">", tradition.as_ref()));
    add_spell_list(&mut page, spells, |(s, _)| s.traditions.contains(&tradition));
    page.push_str("</div>");
    page
}

fn add_spell_list<F>(page: &mut String, spells: &[(Spell, Page)], filter: F)
where
    F: FnMut(&&(Spell, Page)) -> bool,
{
    for (level, spells) in &spells.iter().filter(filter).group_by(|(s, _)| s.level()) {
        page.push_str(&format!(
            "<h2>{}</h2><hr/><table class=\"overview\"><thead><tr><td>Name</td><td class=\"traitcolumn\">Traits</td><td>Description</td></tr></thead>",
            spell_level_as_string(level)
        ));
        for (spell, _) in spells {
            let description = spell
                .description
                .lines()
                // Filter Area, Trigger, etc which are sometimes part of the spell description
                .find(|l| !l.starts_with("<p><strong>") && !l.starts_with("<strong>") && !l.starts_with("<b>") && !l.starts_with("<hr"))
                .map(|l| HTML_FORMATTING_TAGS.replace_all(l, " ").split(". ").next().unwrap_or("").to_owned())
                .unwrap_or_default();
            page.push_str("<tr><td><a href=\"");
            page.push_str(&spell.url_name());
            page.push_str("\">");
            page.push_str(spell.name());
            page.push_str("</a> ");
            if let Some(t) = get_action_img(&spell.time) {
                page.push_str(t);
            }
            page.push_str("</td><td class=\"traitcolumn\">");
            render_traits_inline(page, &spell.traits);
            page.push_str("</td><td>");
            if !description.is_empty() {
                page.push_str(&description);
                page.push('.');
            }
            page.push_str("</td></tr>");
        }
        page.push_str("</table>")
    }
}

fn spell_level_as_string(n: i32) -> &'static str {
    match n {
        0 => "Cantrip",
        1 => "1st Level",
        2 => "2nd Level",
        3 => "3rd Level",
        4 => "4th Level",
        5 => "5th Level",
        6 => "6th Level",
        7 => "7th Level",
        8 => "8th Level",
        9 => "9th Level",
        10 => "10th Level",
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::attach_page;
    use crate::html::Template;
    use crate::tests::read_test_file;
    use crate::tests::DESCRIPTIONS;

    #[test]
    fn spell_list_test() {
        let heal: Spell = serde_json::from_str(&read_test_file("spells.db/heal.json")).expect("Deserialization of heal failed");
        let resurrect: Spell =
            serde_json::from_str(&read_test_file("spells.db/resurrect.json")).expect("Deserialization of resurrect failed");
        let spells = vec![heal, resurrect]
            .into_iter()
            .map(|s| attach_page(s, &DESCRIPTIONS))
            .collect_vec();
        let expected = std::fs::read_to_string("tests/html/spell_list.html").expect("Could not read expected spell list");
        assert_eq!(
            render_full_spell_list(&spells).lines().collect::<String>(),
            expected.lines().collect::<String>(),
        );
    }

    // Gone so we can run on stable. Uncomment if needed.
    /*
    #[bench]
    fn bench_new_spell_template(b: &mut test::Bencher) {
        let raw = fs::read_to_string("foundry/packs/data/spells.db/heal.json").expect("Could not find file");
        let heal: Spell = serde_json::from_str(&raw).expect("Deserialization failed");
        let descriptions = &crate::tests::DESCRIPTIONS;
        b.iter(|| {
            test::black_box(heal.render(&descriptions).len());
        })
    }
    */

    #[test]
    fn test_spell_template() {
        let heal: Spell = serde_json::from_str(&read_test_file("spells.db/heal.json")).expect("Deserialization failed");
        let heal = render_spell(&heal, &DESCRIPTIONS).replace('\n', "");
        let expected = include_str!("../../tests/html/heal.html");
        assert_eq!(heal.lines().collect::<String>(), expected.lines().collect::<String>());
    }

    #[test]
    fn test_spell_template2() {
        let res: Spell = serde_json::from_str(&read_test_file("spells.db/resurrect.json")).expect("Deserialization failed");
        let res = res.render(&DESCRIPTIONS);
        let expected = include_str!("../../tests/html/resurrect.html");
        assert_eq!(res.lines().collect::<String>(), expected.lines().collect::<String>());
    }
}
