#[macro_use]
extern crate enum_display_derive;
use crate::data::actions::Action;
use crate::data::ancestries::Ancestry;
use crate::data::archetypes::Archetype;
use crate::data::backgrounds::Background;
use crate::data::class_features::ClassFeature;
use crate::data::classes::Class;
use crate::data::conditions::Condition;
use crate::data::deities::Deity;
use crate::data::equipment::Equipment;
use crate::data::feats::Feat;
use crate::data::spells::Spell;
use crate::data::traits::{read_trait_descriptions, render_traits};
use crate::data::ObjectName;
use crate::html::render;
use data::HasName;
use futures::executor::block_on;
use itertools::Itertools;
use lazy_static::lazy_static;
use meilisearch_sdk::client::*;
use regex::{Captures, Regex};

mod data;
mod html;

lazy_static! {
    static ref TRAIT_REGEX: Regex = Regex::new(&format!(r"\s({})\s", &read_trait_descriptions(&format!("{}/static/lang/en.json", get_data_path())).0.keys().map(|k| k.to_lowercase()).join("|"))).unwrap();
    static ref DATA_PATH: String = std::env::args().nth(1).unwrap_or_else(|| String::from("foundry"));
    static ref REFERENCE_REGEX: Regex = Regex::new(r"@Compendium\[pf2e\.(.*?)\.(.*?)\]\{(.*?)}").unwrap();
    static ref LEGACY_INLINE_ROLLS: Regex = Regex::new(r"\[\[/r (\d*d?\d+(\+\d+)?) ?(#[\w ]+)?\]\]").unwrap();
    static ref INLINE_ROLLS: Regex = Regex::new(r"\[\[/r [^\[]+\]\]\{(.*?)\}").unwrap();
    static ref INDEX_REGEX: Regex = Regex::new(r"[^A-Za-z0-9]").unwrap();
    // Things to strip from short description. We can’t just remove all tags because we at least
    // want to keep <a> and probably <em>/<b>
    static ref HTML_FORMATTING_TAGS: Regex = Regex::new("</?(p|br|hr|div|span|h1|h2|h3)[^>]*>").unwrap();
    static ref ACTION_GLYPH_REGEX: Regex = Regex::new("<span class=\"pf2-icon\">([ADTFRadtfr123/]+)</span>").unwrap();
    static ref INLINE_STYLE_REGEX: Regex = Regex::new(r#" style="[^"]+""#).unwrap();
}

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

fn get_data_path() -> &'static str {
    &DATA_PATH
}

macro_rules! render_and_index {
    ($type: ty, $source: literal, $target: literal, $additional: expr, $index: ident) => {
        match render::<$type, _>($source, concat!("output/", $target), $additional) {
            Ok(rendered) => {
                $index
                    .add_or_replace(&rendered.iter().cloned().map(|(_, page)| page).collect_vec(), None)
                    .await
                    .unwrap();
                println!(concat!("Successfully rendered ", $target, " folder"));
                rendered
            }
            Err(e) => {
                eprintln!(concat!("Error while rendering ", $target, "folder : {}"), e);
                vec![]
            }
        }
    };
}

fn main() {
    block_on(async move {
        let client = Client::new("http://localhost:7700", &std::env::var("MEILI_KEY").unwrap_or_default());
        let search_index = client.get_or_create("all").await.unwrap();
        // This sets the priority for searching
        search_index
            .set_searchable_attributes(["name", "category", "content"])
            .await
            .unwrap();
        search_index
            .set_displayed_attributes(["name", "category", "content"])
            .await
            .unwrap();
        let descriptions = read_trait_descriptions(&format!("{}/static/lang/en.json", get_data_path()));

        match render_traits("output/trait", &descriptions) {
            Ok(_) => println!("Successfully rendered descriptions"),
            Err(e) => eprintln!("Error while rendering descriptions: {}", e),
        }

        render_and_index!(Feat, "feats.db", "feat", &descriptions, search_index);
        render_and_index!(Spell, "spells.db", "spell", &descriptions, search_index);
        render_and_index!(Background, "backgrounds.db", "background", (), search_index);
        render_and_index!(Archetype, "archetypes.db", "archetype", (), search_index);
        render_and_index!(Action, "actions.db", "action", (), search_index);
        render_and_index!(Condition, "conditionitems.db", "condition", (), search_index);
        render_and_index!(Deity, "deities.db", "deity", (), search_index);
        let classfeatures = render_and_index!(ClassFeature, "classfeatures.db", "classfeature", &descriptions, search_index);
        render_and_index!(Class, "classes.db", "class", &classfeatures, search_index);
        render_and_index!(Equipment, "equipment.db", "item", &descriptions, search_index);
        render_and_index!(Ancestry, "ancestries.db", "ancestry", (), search_index);
    });
}

fn text_cleanup(text: &str, remove_styling: bool) -> String {
    let resolved_references = REFERENCE_REGEX.replace_all(text, |caps: &Captures| {
        // These are compendium items only used for automation in foundry,
        // so they don’t contain meaningless links.
        if caps[1].ends_with("-effects") || &caps[1] == "pf2e-macros" {
            if caps[2].starts_with("Effect:") {
                String::new()
            } else {
                caps[2].to_string()
            }
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
                "equipment-srd" => "item",
                // unsure, maybe these should just both be features?
                "ancestryfeatures" => "ancestryfeature",
                "classfeatures" => "classfeature",
                "hazards" => "hazard", // Should these be creatures?
                "bestiary-ability-glossary-srd" => "creature_abilities",
                "familiar-abilities" => "familiar_abilities",
                "archetypes" => "archetype",
                "backgrounds" => "background",
                "deities" => "deity",
                "rollable-tables" => "table",
                c => unimplemented!("{}", c),
            };
            let element = ObjectName(&caps[2]);
            format!(r#"<a href="/{}/{}">{}</a>"#, category, element.url_name(), &caps[3])
        }
    });
    let clean_rolls = &INLINE_ROLLS.replace_all(&resolved_references, |caps: &Captures| caps[1].to_string());
    let resolved_icons = LEGACY_INLINE_ROLLS.replace_all(clean_rolls, |caps: &Captures| caps[1].to_string());
    let replaced_references = &ACTION_GLYPH_REGEX.replace_all(&resolved_icons, |caps: &Captures| {
        let mut replacement = String::from(" ");
        replacement.push_str(get_action_img(&caps[1]).unwrap_or(""));
        replacement
    });
    if remove_styling {
        INLINE_STYLE_REGEX.replace_all(replaced_references, "").to_string()
    } else {
        replaced_references.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::traits::TraitDescriptions;

    use super::*;
    use std::fs;

    pub fn read_test_file(path: &str) -> String {
        fs::read_to_string(format!("foundry/packs/data/{}", path)).expect("Could not find file")
    }
    lazy_static! {
        pub static ref DESCRIPTIONS: TraitDescriptions = read_trait_descriptions(&format!("foundry/static/lang/en.json"));
    }

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
        assert_eq!(text_cleanup(input, true), expected);

        let input =
            "[[/r ceil(@details.level.value/2)d8 #piercing]]{Levelled} piercing damage and [[/r 123 #something]]{123 something} damage";
        let expected = "Levelled piercing damage and 123 something damage";
        assert_eq!(INLINE_ROLLS.replace_all(input, |caps: &Captures| caps[1].to_string()), expected);
    }

    #[test]
    fn legacy_inline_roll_test() {
        let input = "Freezing sleet and heavy snowfall collect on the target's feet and legs, dealing [[/r 1d4 #cold]] cold damage.";
        let expected = "Freezing sleet and heavy snowfall collect on the target's feet and legs, dealing 1d4 cold damage.";
        assert_eq!(text_cleanup(input, true), expected);

        let input = "Increase the damage to fire creatures by [[/r 2d8]].";
        let expected = "Increase the damage to fire creatures by 2d8.";
        assert_eq!(text_cleanup(input, false), expected);
    }
}
