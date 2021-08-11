#[macro_use]
extern crate enum_display_derive;
use crate::data::archetypes::Archetype;
use crate::data::backgrounds::Background;
use crate::data::conditions::Condition;
use crate::data::deities::Deity;
use crate::data::traits::read_trait_descriptions;
use crate::data::ObjectName;
use crate::html::actions::{render_action_list, ActionTemplate};
use crate::html::feats::FeatTemplate;
use crate::html::spells::{render_spell_list, SpellTemplate};
use askama::Template;
use data::HasName;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use serde::Deserialize;
use std::io::BufReader;
use std::{fs, io};

mod data;
mod html;

lazy_static! {
    static ref DATA_PATH: String = std::env::args().nth(1).expect("Expected path to foundry module root");
    static ref REFERENCE_REGEX: Regex = Regex::new(r"@Compendium\[pf2e\.(.*?)\.(.*?)\]\{(.*?)}").unwrap();
    static ref LEGACY_INLINE_ROLLS: Regex = Regex::new(r"\[\[/r (\d*d?\d+(\+\d+)?) ?(#[\w ]+)?\]\]").unwrap();
    static ref INLINE_ROLLS: Regex = Regex::new(r"\[\[/r [^\[]+\]\]\{(.*?)\}").unwrap();
    // Things to strip from short description. We can’t just remove all tags because we at least
    // want to keep <a> and probably <em>/<b>
    static ref HTML_FORMATTING_TAGS: Regex = Regex::new("</?(p|br|hr|div|span)>").unwrap();
}

fn get_action_img(val: &str) -> &str {
    match val {
        "1" => r#"<img alt="One Action" class="actionimage" src="/static/actions/OneAction.webp">"#,
        "2" => r#"<img alt="Two Actions" class="actionimage" src="/static/actions/TwoActions.webp">"#,
        "3" => r#"<img alt="Three Actions" class="actionimage" src="/static/actions/ThreeActions.webp">"#,
        "1 or 2" => {
            r#"<img alt="One Action" class="actionimage" src="/static/actions/OneAction.webp"> or <img alt="Two Actions" class="actionimage" src="/static/actions/TwoActions.webp">"#
        }
        "1 to 3" => {
            r#"<img alt="One Action" class="actionimage" src="/static/actions/OneAction.webp"> to <img alt="Three Actions" class="actionimage" src="/static/actions/ThreeActions.webp">"#
        }
        "2 or 3" => {
            r#"<img alt="Two Actions" class="actionimage" src="/static/actions/TwoActions.webp"> or <img alt="Three Actions" class="actionimage" src="/static/actions/ThreeActions.webp">"#
        }
        "free" => r#"<img alt="Free Action" class="actionimage" src="/static/actions/FreeAction.webp">"#,
        "reaction" => r#"<img alt="Reaction" class="actionimage" src="/static/actions/Reaction.webp">"#,
        "passive" => r#"<img alt="Passive" class="actionimage" src="/static/actions/Passive.webp">"#, // Check if this is used anywhere
        val => val,
    }
}

fn get_data_path() -> &'static str {
    &DATA_PATH
}

fn main() {
    let descriptions = read_trait_descriptions(&format!("{}/static/lang/en.json", get_data_path()));
    match render_category("feats.db", "output/feat", &descriptions, FeatTemplate::new) {
        Ok(_) => println!("Successfully rendered feats"),
        Err(e) => eprintln!("Error while rendering feats: {}", e),
    }
    match render_category("spells.db", "output/spell", &descriptions, SpellTemplate::new) {
        Ok(_) => println!("Successfully rendered spells"),
        Err(e) => eprintln!("Error while rendering spells: {}", e),
    }
    match render_category("deities.db", "output/deity", &descriptions, |deity: Deity, _| Deity {
        content: replace_references(&deity.content),
        ..deity
    }) {
        Ok(_) => println!("Successfully rendered deities"),
        Err(e) => eprintln!("Error while rendering deities: {}", e),
    }
    match render_category("backgrounds.db", "output/background", &(), |bg: Background, _| bg) {
        Ok(_) => println!("Successfully rendered backgounds"),
        Err(e) => eprintln!("Error while rendering backgounds: {}", e),
    }
    match render_category("conditionitems.db", "output/condition", &descriptions, |c: Condition, _| c) {
        Ok(_) => println!("Successfully rendered conditions"),
        Err(e) => eprintln!("Error while rendering conditions: {}", e),
    }
    match render_category("archetypes.db", "output/archetype", &(), |at: Archetype, _| Archetype {
        // The first line of each archetype is just the name again, so we skip that
        content: replace_references(&at.content).lines().skip(1).collect(),
        ..at
    }) {
        Ok(_) => println!("Successfully rendered archetypes"),
        Err(e) => eprintln!("Error while rendering archetypes: {}", e),
    }
    match render_category("actions.db", "output/action", &descriptions, ActionTemplate::new) {
        Ok(_) => println!("Successfully rendered actions"),
        Err(e) => eprintln!("Error while rendering actions: {}", e),
    }
    match render_spell_list("spells.db", "output/spell") {
        Ok(_) => println!("Successfully rendered spell index"),
        Err(e) => eprintln!("Error while rendering spell index: {}", e),
    }
    match render_action_list("actions.db", "output/action") {
        Ok(_) => println!("Successfully rendered action index"),
        Err(e) => eprintln!("Error while rendering action index: {}", e),
    }
}

fn render_category<T: for<'de> Deserialize<'de> + HasName + Clone, R: Template, F: FnMut(T, &D) -> R, D>(
    src_path: &str,
    output_path: &str,
    additional_data: &D,
    mut convert: F,
) -> io::Result<()> {
    fs::create_dir_all(output_path)?;
    let mut list = String::with_capacity(100_000);
    list.push_str("<ul>");
    for f in fs::read_dir(&format!("{}/packs/data/{}", get_data_path(), src_path))? {
        let filename = f?.path();
        // println!("Reading {}", filename.to_str().unwrap());
        let f = fs::File::open(&filename)?;
        let reader = BufReader::new(f);
        let object: T = serde_json::from_reader(reader).expect("Deserialization failed");
        let template = convert(object.clone(), additional_data);
        let output_filename = object.url_name();
        let full_output_filename = &format!("{}/{}", output_path, output_filename);
        fs::write(full_output_filename, template.render().expect("Failed to render"))?;
        list.push_str(&format!("<li><a href=\"{}\">{}</a></li>\n", output_filename, object.name()));
    }
    list.push_str("</ul>");
    list.push_str("<div style=\"height: 2em\"></div>");
    list.push_str("<a href=\"/\">Back</a>");
    fs::write(&format!("{}/index.html", output_path), &list)?;
    Ok(())
}

fn replace_references(text: &str) -> String {
    let resolved_references = REFERENCE_REGEX.replace_all(text, |caps: &Captures| {
        // These are compendium items only used for automation in foundry,
        // so we can remove any reference to them.
        if caps[1].ends_with("-effects") || &caps[1] == "pf2e-macros" {
            String::new()
        } else {
            let category = match &caps[1] {
                // There are separate compendia for age-of-ashes-bestiary, abomination-vaults-bestiary, etc.
                // We summarize these under creatures
                cat if cat.contains("-bestiary") => "creature",
                "feats-srd" => "feat",
                "conditionitems" => "condition",
                "spells-srd" => "spell",
                "actionspf2e" => "action",
                "action-macros" => "action", // TODO: check exhaustively if this works
                "equipment-srd" => "equipment",
                // unsure, maybe these should just both be features?
                "ancestryfeatures" => "ancestryfeature",
                "classfeatures" => "classfeature",
                "hazards" => "hazard", // Should these be creatures?
                "bestiary-ability-glossary-srd" => "creature_abilities",
                "familiar-abilities" => "familiar_abilities",
                "archetypes" => "archetype",
                "backgrounds" => "background",
                "deities" => "deity",
                c => unimplemented!("{}", c),
            };
            let element = ObjectName(&caps[2]);
            format!(r#" <a href="/{}/{}">{}</a>"#, category, element.url_name(), &caps[3])
        }
    });
    LEGACY_INLINE_ROLLS
        .replace_all(
            &INLINE_ROLLS.replace_all(&resolved_references, |caps: &Captures| caps[1].to_string()),
            |caps: &Captures| caps[1].to_string(),
        )
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_tag_regex_test() {
        let input = "<p>You perform rapidly, speeding up your ally.</br>";
        let expected = "You perform rapidly, speeding up your ally.";
        assert_eq!(HTML_FORMATTING_TAGS.replace_all(input, ""), expected);
    }

    #[test]
    fn inline_roll_regex_test() {
        let input = "Freezing sleet and heavy snowfall collect on the target's feet and legs, dealing [[/r 1d4 #cold]] cold damage and [[/r 1d6 #persistent bleed]]{1d6 persistent bleed} and [[/r 1 #sad]] sad damage and [[/r 1d1+1 #balumbdar]] balumbdar damage for the unit test.";
        let expected = "Freezing sleet and heavy snowfall collect on the target's feet and legs, dealing 1d4 cold damage and 1d6 persistent bleed and 1 sad damage and 1d1+1 balumbdar damage for the unit test.";
        assert_eq!(replace_references(input), expected);

        let input = "[[/r ceil(@details.level.value/2)d8 #piercing]]{Levelled} piercing damage and [[/r 123 #something]]{123 something} damage";
        let expected = "Levelled piercing damage and 123 something damage";
        assert_eq!(INLINE_ROLLS.replace_all(input, |caps: &Captures| caps[1].to_string()), expected);
    }

    #[test]
    fn legacy_inline_roll_test() {
        let input = "Freezing sleet and heavy snowfall collect on the target's feet and legs, dealing [[/r 1d4 #cold]] cold damage.";
        let expected = "Freezing sleet and heavy snowfall collect on the target's feet and legs, dealing 1d4 cold damage.";
        assert_eq!(replace_references(input), expected);

        let input = "Increase the damage to fire creatures by [[/r 2d8]].";
        let expected = "Increase the damage to fire creatures by 2d8.";
        assert_eq!(replace_references(input), expected);
    }
}
