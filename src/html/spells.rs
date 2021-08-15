use super::render_traits;
use crate::data::spells::{Area, Spell, SpellCategory, SpellTradition};
use crate::data::traits::TraitDescriptions;
use crate::data::HasName;
use crate::html::render_trait_legend;
use crate::{get_action_img, get_data_path, HTML_FORMATTING_TAGS};
use itertools::Itertools;
use std::borrow::Cow;
use std::fs::write;
use std::io::BufReader;
use std::{fs, io};

impl super::Template<&TraitDescriptions> for Spell {
    fn render(&self, trait_descriptions: &TraitDescriptions) -> std::borrow::Cow<'_, str> {
        Cow::Owned(render_spell(self, trait_descriptions))
    }
}

pub fn render_spells(folder: &str, target: &str, trait_descriptions: &TraitDescriptions) -> io::Result<Vec<Spell>> {
    fs::create_dir_all(target)?;
    let mut all_spells = fs::read_dir(&format!("{}/packs/data/{}", get_data_path(), folder))?
        .map(|f| {
            let filename = f?.path();
            // println!("Reading {}", filename.to_str().unwrap());
            let f = fs::File::open(&filename)?;
            let reader = BufReader::new(f);
            let spell: Spell = serde_json::from_reader(reader).expect("Deserialization failed");
            Ok(spell)
        })
        .collect::<io::Result<Vec<Spell>>>()?;
    // Sort first by name and then by level. Don’t use unstable sorting here!
    all_spells.sort_by_key(|s| s.name.clone());
    all_spells.sort_by_key(|s| if s.is_cantrip() { 0 } else { s.level });
    render_spell_lists(&all_spells, target)?;
    for spell in &all_spells {
        write(format!("{}/{}", target, spell.url_name()), render_spell(spell, trait_descriptions))?;
    }
    Ok(all_spells)
}

fn render_spell(spell: &Spell, trait_descriptions: &TraitDescriptions) -> String {
    let mut page = String::with_capacity(4000);
    page.push_str(&format!(
        r#"<h1>{}<span class="type">{} {}</span></h1><hr/>"#,
        spell.name(),
        if spell.is_cantrip() {
            SpellCategory::Cantrip
        } else {
            spell.category
        },
        spell.level,
    ));
    page = render_traits(page, &spell.traits);
    if !spell.traditions.is_empty() {
        page.push_str("<b>Traditions</b> ");
        page.push_str(&spell.traditions.iter().map(SpellTradition::to_string).join(", "));
        page.push_str("<br/>");
    }
    page.push_str(&format!("<b>Cast</b> {}<br/>", get_action_img(&spell.time)));
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
        page.push_str(&save.to_string());
        page.push_str("<br/>");
    }
    page.push_str("<hr/>");
    page.push_str(&spell.description);
    page.push_str("<hr/>");
    render_trait_legend(page, &spell.traits, trait_descriptions)
}

fn render_spell_lists(all_spells: &[Spell], target: &str) -> io::Result<()> {
    fs::write(
        format!("{}/{}", target, "arcane"),
        render_tradition(all_spells, SpellTradition::Arcane),
    )?;
    fs::write(
        format!("{}/{}", target, "divine"),
        render_tradition(all_spells, SpellTradition::Divine),
    )?;
    fs::write(
        format!("{}/{}", target, "occult"),
        render_tradition(all_spells, SpellTradition::Occult),
    )?;
    fs::write(
        format!("{}/{}", target, "primal"),
        render_tradition(all_spells, SpellTradition::Primal),
    )?;
    fs::write(format!("{}/index.html", target), render_full_spell_list(all_spells))
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

fn render_full_spell_list(spells: &[Spell]) -> String {
    let mut page = String::with_capacity(100_000);
    page = add_spell_header(page);
    page.push_str("<h1>All Spells</h1><hr><br/><div id=\"spelllist\">");
    page = add_spell_list(page, spells, |_| true);
    page.push_str("</div>");
    page
}

fn render_tradition(spells: &[Spell], tradition: SpellTradition) -> String {
    let mut page = String::with_capacity(50_000);
    page = add_spell_header(page);
    page.push_str(&format!("<h1>{} Spell List</h1><hr><br/><div id=\"spelllist\">", tradition));
    page = add_spell_list(page, spells, |s| s.traditions.contains(&tradition));
    page.push_str("</div>");
    page
}

fn add_spell_list<F>(mut page: String, spells: &[Spell], filter: F) -> String
where
    F: FnMut(&&Spell) -> bool,
{
    for (level, spells) in &spells.iter().filter(filter).group_by(|s| if s.is_cantrip() { 0 } else { s.level }) {
        page.push_str(&format!("<h2>{}</h2><hr>", spell_level_as_string(level)));
        for spell in spells {
            let description = spell
                .description
                .lines()
                // Filter Area, Trigger, etc which are sometimes part of the spell description
                .find(|l| !l.starts_with("<p><strong>") && !l.starts_with("<strong>") && !l.starts_with("<b>") && !l.starts_with("<hr"))
                .map(|l| HTML_FORMATTING_TAGS.replace_all(l, " ").split(". ").next().unwrap_or("").to_owned())
                .unwrap_or_default();
            page.push_str("<p><a href=\"");
            page.push_str(&spell.url_name());
            page.push_str("\">");
            page.push_str(spell.name());
            page.push_str("</a> (");
            page.push_str(&spell.school.to_string());
            page.push_str("): ");
            if !description.is_empty() {
                page.push_str(&description);
                page.push('.');
            }
            page.push_str("</p>");
        }
    }
    page
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
    use crate::html::Template;
    use crate::tests::read_test_file;
    use crate::tests::DESCRIPTIONS;

    #[test]
    fn spell_list_test() {
        let heal: Spell = serde_json::from_str(&read_test_file("spells.db/heal.json")).expect("Deserialization of heal failed");
        let resurrect: Spell =
            serde_json::from_str(&read_test_file("spells.db/resurrect.json")).expect("Deserialization of resurrect failed");
        let spells = vec![heal, resurrect];
        let expected = std::fs::read_to_string("tests/html/spell_list.html").expect("Could not read expected spell list");
        assert_eq!(
            render_full_spell_list(&spells).lines().collect::<String>(),
            expected.lines().collect::<String>(),
        );
    }

    #[bench]
    fn bench_new_spell_template(b: &mut test::Bencher) {
        let raw = fs::read_to_string("foundry/packs/data/spells.db/heal.json").expect("Could not find file");
        let heal: Spell = serde_json::from_str(&raw).expect("Deserialization failed");
        let descriptions = &crate::tests::DESCRIPTIONS;
        b.iter(|| {
            test::black_box(heal.render(&descriptions).len());
        })
    }

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
