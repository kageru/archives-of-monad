use super::render_traits;
use crate::{
    data::{
        spells::{Area, Spell, SpellCategory, SpellTradition},
        traits::Translations,
        HasLevel, HasName,
    },
    html::{render_trait_legend, render_traits_inline, write_full_html_document, HtmlPage, Template},
    HTML_FORMATTING_TAGS,
};
use itertools::Itertools;
use std::{borrow::Cow, fmt::Write, io};

impl Template<&Translations> for Spell {
    fn render(&self, trait_descriptions: &Translations) -> String {
        render_spell(self, trait_descriptions)
    }

    fn render_subindices(target: &str, elements: &[(Self, HtmlPage)]) -> io::Result<()> {
        write_full_html_document(
            &format!("{}/{}", target, "arcane"),
            "Arcane Spells",
            &render_tradition(elements, SpellTradition::Arcane),
        )?;
        write_full_html_document(
            &format!("{}/{}", target, "divine"),
            "Divine Spells",
            &render_tradition(elements, SpellTradition::Divine),
        )?;
        write_full_html_document(
            &format!("{}/{}", target, "occult"),
            "Occult Spells",
            &render_tradition(elements, SpellTradition::Occult),
        )?;
        write_full_html_document(
            &format!("{}/{}", target, "primal"),
            "Primal Spells",
            &render_tradition(elements, SpellTradition::Primal),
        )?;

        for t in elements.iter().flat_map(|(s, _)| &s.traits.misc).unique() {
            let mut page = String::with_capacity(250_000);
            page = add_spell_header(page);
            write!(page, "<h1>{} Spells</h1><hr><br/>", t);
            add_spell_list(&mut page, elements, |(s, _)| s.traits.misc.contains(t));
            write_full_html_document(&format!("{}/trait_{}", target, t.to_lowercase()), &format!("{} Spells", t), &page)?;
        }
        Ok(())
    }

    fn render_index(elements: &[(Self, HtmlPage)]) -> String {
        render_full_spell_list(elements)
    }

    fn category(&self) -> String {
        Cow::Borrowed(if self.is_cantrip() { SpellCategory::Cantrip } else { self.category }.into()).to_string()
    }
}

// TODO: dedup this with the strings in [parser]
fn get_action_img(val: &str) -> Option<&'static str> {
    match val {
        "1" | "A" | "a" => Some(r#"<img alt="One Action" class="actionimage" src="/static/actions/OneAction.webp">"#),
        "2" | "D" | "d" => Some(r#"<img alt="Two Actions" class="actionimage" src="/static/actions/TwoActions.webp">"#),
        "3" | "T" | "t" => Some(r#"<img alt="Three Actions" class="actionimage" src="/static/actions/ThreeActions.webp">"#),
        "1 or 2" | "A/D" => Some(
            r#"<img alt="One Action" class="actionimage" src="/static/actions/OneAction.webp"> or <img alt="Two Actions" class="actionimage" src="/static/actions/TwoActions.webp">"#,
        ),
        "1 to 3" | "A/T" => Some(
            r#"<img alt="One Action" class="actionimage" src="/static/actions/OneAction.webp"> to <img alt="Three Actions" class="actionimage" src="/static/actions/ThreeActions.webp">"#,
        ),
        "2 or 3" | "D/T" => Some(
            r#"<img alt="Two Actions" class="actionimage" src="/static/actions/TwoActions.webp"> or <img alt="Three Actions" class="actionimage" src="/static/actions/ThreeActions.webp">"#,
        ),
        "free" | "F" | "f" => Some(r#"<img alt="Free Action" class="actionimage" src="/static/actions/FreeAction.webp">"#),
        "reaction" | "R" | "r" => Some(r#"<img alt="Reaction" class="actionimage" src="/static/actions/Reaction.webp">"#),
        "passive" => Some(r#"<img alt="Passive" class="actionimage" src="/static/actions/Passive.webp">"#), // Check if this is used anywhere
        _ => None,
    }
}

fn render_spell(spell: &Spell, trait_descriptions: &Translations) -> String {
    let mut page = String::with_capacity(4000);
    write!(
        page,
        r#"<h1><a href="/spell/{}">{}</a><span class="type">{} {}</span></h1><hr/>"#,
        spell.url_name(),
        spell.name(),
        spell.category(),
        spell.level,
    );
    render_traits(&mut page, &spell.traits);
    write!(page, "<b>Source</b> {}<br/>", &spell.source);
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
        write!(page, "<b>Cost</b> {}<br/>", &spell.cost);
    }
    if !spell.secondary_casters.is_empty() {
        write!(page, "<b>Secondary Casters</b> {}<br/>", &spell.secondary_casters);
    }
    if !spell.primary_check.is_empty() {
        write!(page, "<b>Primary Check</b> {}<br/>", &spell.primary_check);
    }
    if !spell.secondary_check.is_empty() {
        write!(page, "<b>Secondary Checks</b> {}<br/>", &spell.secondary_check);
    }
    match (&spell.area_string, spell.area) {
        (Some(area), _) => {
            write!(page, "<b>Area</b> {}<br/>", area);
        }
        (None, area) if area != Area::None => {
            write!(page, "<b>Area</b> {}<br/>", area);
        }
        _ => (),
    }
    if !spell.range.is_empty() {
        write!(page, "<b>Range</b> {}<br/>", &spell.range);
    }
    if !spell.target.is_empty() {
        write!(page, "<b>Target</b> {}<br/>", &spell.target);
    }
    if !spell.duration.is_empty() {
        write!(page, "<b>Duration</b> {}<br/>", &spell.duration);
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

fn render_full_spell_list(spells: &[(Spell, HtmlPage)]) -> String {
    let mut page = String::with_capacity(100_000);
    page = add_spell_header(page);
    page.push_str("<h1>All Spells</h1><hr/><br/>");
    add_spell_list(&mut page, spells, |_| true);
    page
}

fn render_tradition(spells: &[(Spell, HtmlPage)], tradition: SpellTradition) -> String {
    let mut page = String::with_capacity(50_000);
    page = add_spell_header(page);
    write!(page, "<h1>{} Spell List</h1><hr/><br/><div id=\"list\">", tradition.as_ref());
    add_spell_list(&mut page, spells, |(s, _)| s.traditions.contains(&tradition));
    page.push_str("</div>");
    page
}

fn add_spell_list<F>(page: &mut String, spells: &[(Spell, HtmlPage)], filter: F)
where
    F: FnMut(&&(Spell, HtmlPage)) -> bool,
{
    for (level, spells) in &spells.iter().filter(filter).group_by(|(s, _)| s.level()) {
        write!(page,
               "<h2>{}</h2><hr/><table class=\"overview\"><thead><tr><td>Name</td><td class=\"traitcolumn\">Traits</td><td>Description</td></tr></thead>",
               spell_level_as_string(level)
        );
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

pub fn spell_level_as_string(n: i32) -> &'static str {
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
        11 => "11th Level", // exists in foundry for reasons
        _ => unreachable!("Level {}", n),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::attach_html;
    use crate::html::Template;
    use crate::tests::assert_eq_ignore_linebreaks;
    use crate::tests::read_test_file;
    use crate::tests::TRANSLATIONS;

    #[test]
    fn spell_list_test() {
        let heal: Spell = serde_json::from_str(&read_test_file("spells/heal.json")).expect("Deserialization of heal failed");
        let resurrect: Spell = serde_json::from_str(&read_test_file("spells/resurrect.json")).expect("Deserialization of resurrect failed");
        let spells = vec![heal, resurrect]
            .into_iter()
            .map(|s| attach_html(s, &TRANSLATIONS))
            .collect_vec();
        assert_eq_ignore_linebreaks(&render_full_spell_list(&spells), include_str!("../../tests/html/spell_list.html"));
    }

    #[test]
    fn test_spell_template() {
        let heal: Spell = serde_json::from_str(&read_test_file("spells/heal.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&render_spell(&heal, &TRANSLATIONS), include_str!("../../tests/html/heal.html"));
    }

    #[test]
    fn test_spell_template2() {
        let res: Spell = serde_json::from_str(&read_test_file("spells/resurrect.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&res.render(&TRANSLATIONS), include_str!("../../tests/html/resurrect.html"));
    }
}
