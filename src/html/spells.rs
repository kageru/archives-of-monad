use super::super::get_action_img;
use crate::data::damage::{Damage, DamageScaling, DamageType};
use crate::data::spells::{Area, Save, Spell, SpellCategory, SpellComponents, SpellSchool, SpellTradition, SpellType};
use crate::data::traits::{Rarity, Trait, TraitDescriptions};
use crate::data::HasName;
use crate::{get_data_path, HTML_FORMATTING_TAGS};
use askama::Template;
use convert_case::{Case, Casing};
use itertools::Itertools;
use std::io::BufReader;
use std::{fs, io};

// Ideally, this wouldn’t parse all of the spells again,
// but it’s good enough for now
pub fn render_spell_list(folder: &str, target: &str) -> io::Result<()> {
    let mut all_spells = fs::read_dir(&format!("{}/packs/data/{}", get_data_path(), folder))?
        .filter_map(|f| {
            let filename = f.ok()?.path();
            // println!("Reading {}", filename.to_str().unwrap());
            let f = fs::File::open(&filename).ok()?;
            let reader = BufReader::new(f);
            let spell: Spell = serde_json::from_reader(reader).expect("Deserialization failed");
            Some(spell)
        })
        .collect_vec();
    // Sort first by name and then by level. Don’t use unstable sorting here!
    all_spells.sort_by_key(|s| s.name.clone());
    all_spells.sort_by_key(|s| if s.is_cantrip() { 0 } else { s.level });
    fs::write(
        format!("{}/{}", target, "arcane"),
        render_tradition(&all_spells, SpellTradition::Arcane),
    )?;
    fs::write(
        format!("{}/{}", target, "divine"),
        render_tradition(&all_spells, SpellTradition::Divine),
    )?;
    fs::write(
        format!("{}/{}", target, "occult"),
        render_tradition(&all_spells, SpellTradition::Occult),
    )?;
    fs::write(
        format!("{}/{}", target, "primal"),
        render_tradition(&all_spells, SpellTradition::Primal),
    )?;
    fs::write(format!("{}/index.html", target), render_full_spell_list(&all_spells))
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

fn add_spell_list<F>(mut page: String, spells: &[Spell], filter: F) -> String
where
    F: FnMut(&&Spell) -> bool,
{
    for (level, spells) in &spells.iter().filter(filter).group_by(|s| if s.is_cantrip() { 0 } else { s.level }) {
        page.push_str(&format!("<h2>{}</h2><hr>", english_number(level)));
        page.push_str("<p>");
        for spell in spells {
            let description = spell
                .description
                .lines()
                // Filter Area, Trigger, etc which are sometimes part of the spell description
                .find(|l| !l.starts_with("<p><strong>") && !l.starts_with("<strong>") && !l.starts_with("<b>") && !l.starts_with("<hr"))
                .map(|l| HTML_FORMATTING_TAGS.replace_all(l, " ").split(". ").next().unwrap_or("").to_owned())
                .unwrap_or_default();
            page.push_str(&format!(
                r#"<p><a href="{}">{}</a> ({}): {}{}</p>"#,
                &spell.url_name(),
                spell.name(),
                spell.school,
                description,
                // We split on .
                // If, for some reason, there is no . and the description is empty,
                // don’t re-add the . here.
                if description.is_empty() { "" } else { "." }
            ));
        }
        page.push_str("</p>");
    }
    page
}

// Should this be a proper template instead?
fn render_full_spell_list(spells: &[Spell]) -> String {
    let mut page = String::with_capacity(100_000);
    page = add_spell_header(page);
    page.push_str("<h1>All Spells</h1><hr></br>");
    add_spell_list(page, spells, |_| true)
}

fn render_tradition(spells: &[Spell], tradition: SpellTradition) -> String {
    let mut page = String::with_capacity(50_000);
    page = add_spell_header(page);
    page.push_str(&format!("<h1>{} Spell List</h1><hr></br>", tradition));
    add_spell_list(page, spells, |s| s.traditions.contains(&tradition))
}

fn english_number(n: i32) -> &'static str {
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

#[derive(Template, PartialEq, Debug)]
#[template(path = "spell.html", escape = "none")]
pub struct SpellTemplate {
    pub name: String,
    pub area: Area,
    pub area_string: Option<String>, // not happy with this
    pub basic_save: bool,
    pub components: SpellComponents,
    pub cost: String,
    pub category: SpellCategory,
    pub damage: Damage,
    pub damage_type: DamageType,
    pub description: String,
    pub duration: String,
    pub level: i32,
    pub range: String,
    pub save: Option<Save>,
    pub scaling: DamageScaling,
    pub school: SpellSchool,
    pub secondary_casters: String,
    pub secondary_check: String,
    pub spell_type: SpellType,
    pub sustained: bool,
    pub target: String,
    pub time: String,
    pub primary_check: String,
    pub traditions: Vec<SpellTradition>,
    pub traits: Vec<Trait>,
    pub rarity: Option<(Rarity, String)>,
}

impl SpellTemplate {
    pub fn new(spell: Spell, trait_descriptions: &TraitDescriptions) -> Self {
        let spell_category = if spell.is_cantrip() {
            SpellCategory::Cantrip
        } else {
            spell.category
        };

        let test = spell
            .traits
            .value
            .iter()
            .map(|name| name.to_case(Case::Pascal))
            .map(|name| Trait {
                description: trait_descriptions
                    .0
                    .get(&name)
                    .cloned()
                    .unwrap_or_else(|| String::from("NOT_FOUND")),
                name,
            })
            .collect();

        SpellTemplate {
            name: spell.name,
            area: spell.area,
            area_string: spell.area_string,
            basic_save: spell.basic_save,
            components: spell.components,
            cost: spell.cost,
            category: spell_category,
            damage: spell.damage,
            damage_type: spell.damage_type,
            description: spell.description,
            duration: spell.duration,
            level: spell.level,
            range: spell.range,
            save: spell.save,
            scaling: spell.scaling,
            school: spell.school,
            secondary_casters: spell.secondary_casters,
            secondary_check: spell.secondary_check,
            spell_type: spell.spell_type,
            sustained: spell.sustained,
            target: spell.target,
            time: get_action_img(&spell.time).to_string(),
            primary_check: spell.primary_check,
            traditions: spell.traditions,
            traits: test,
            rarity: spell.traits.rarity.map(|r| (r, trait_descriptions.0[&r.to_string()].clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::read_test_file;

    use super::*;

    #[test]
    fn spell_list_test() {
        let heal: Spell = serde_json::from_str(&read_test_file("spells.db/heal.json")).expect("Deserialization of heal failed");
        let resurrect: Spell =
            serde_json::from_str(&read_test_file("spells.db/resurrect.json")).expect("Deserialization of resurrect failed");
        let spells = vec![heal, resurrect];
        let expected = std::fs::read_to_string("tests/html/spell_list.html").expect("Could not read expected spell list");
        assert_eq!(render_full_spell_list(&spells), expected);
    }
}
