#[macro_use]
extern crate strum;
use crate::data::creature::Npc;
use data::{
    actions::Action,
    ancestries::Ancestry,
    archetypes::Archetype,
    backgrounds::Background,
    class_features::ClassFeature,
    classes::Class,
    conditions::Condition,
    deities::Deity,
    equipment::Equipment,
    feats::Feat,
    spells::Spell,
    traits::{read_trait_descriptions, render_traits},
    HasName, ObjectName,
};
use futures::executor::block_on;
use html::render;
use itertools::Itertools;
use lazy_static::lazy_static;
use meilisearch_sdk::client::*;
use regex::{Captures, Regex};
use std::{fs, io};

mod data;
mod html;

lazy_static! {
    static ref DATA_PATH: String = std::env::args().nth(1).unwrap_or_else(|| String::from("foundry"));

    static ref TRAIT_REGEX: Regex = Regex::new(&format!(r"\s({})\s", &read_trait_descriptions(&format!("{}/static/lang/en.json", get_data_path())).0.keys().map(|k| k.to_lowercase()).join("|"))).unwrap();
    static ref REFERENCE_REGEX: Regex = Regex::new(r"@Compendium\[pf2e\.(.*?)\.(.*?)\]\{(.*?)}").unwrap();
    static ref LEGACY_INLINE_ROLLS: Regex = Regex::new(r"\[\[/r (\d*d?\d+(\+\d+)?) ?(#[\w ]+)?\]\]").unwrap();
    static ref INLINE_ROLLS: Regex = Regex::new(r"\[\[/b?r [^\[]+\]\]\{(.*?)\}").unwrap();
    static ref INDEX_REGEX: Regex = Regex::new(r"[^A-Za-z0-9]").unwrap();
    // Things to strip from short description. We can’t just remove all tags because we at least
    // want to keep <a> and probably <em>/<b>
    static ref HTML_FORMATTING_TAGS: Regex = Regex::new("</?(p|br|hr|div|span|h1|h2|h3)[^>]*>").unwrap();
    static ref ACTION_GLYPH_REGEX: Regex = Regex::new("<span class=\"pf2-icon\">([ADTFRadtfr123/]+)</span>").unwrap();
    static ref INLINE_STYLE_REGEX: Regex = Regex::new(r#" style="[^"]+""#).unwrap();
    static ref APPLIED_EFFECTS_REGEX: Regex = Regex::new("(<hr ?/>\n?)?<p>Automatically applied effects:</p>\n?<ul>(.|\n)*</ul>").unwrap();
    static ref INLINE_SAVES_REGEX: Regex = Regex::new(r#"<span [^>]*data-pf2-dc=" ?(\d+) ?"[^>]*>([a-zA-Z0-9 -]+)</span>"#).unwrap();
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
    ($type: ty, $source: expr, $target: literal, $additional: expr, $index: ident) => {
        match render::<$type, _, _>(&$source, concat!("output/", $target), $additional) {
            Ok(rendered) => {
                if let Some(index) = &$index {
                    if let Err(e) = index
                        .add_or_replace(&rendered.iter().cloned().map(|(_, page)| page).collect_vec(), None)
                        .await
                    {
                        eprintln!("Could not update meilisearch index: {:?}", e);
                    }
                }
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
        let search_index = build_search_index().await;

        let descriptions = read_trait_descriptions(&format!("{}/static/lang/en.json", get_data_path()));
        match (render_traits("output/trait", &descriptions), &search_index) {
            (Ok(traits), Some(index)) => {
                index.add_or_replace(&traits, None).await.unwrap();
            }
            (Ok(_), None) => println!("Successfully rendered descriptions"),
            (Err(e), _) => eprintln!("Error while rendering descriptions: {}", e),
        }

        render_and_index!(Feat, ["feats.db"], "feat", &descriptions, search_index);
        render_and_index!(Spell, ["spells.db"], "spell", &descriptions, search_index);
        render_and_index!(Background, ["backgrounds.db"], "background", (), search_index);
        render_and_index!(Archetype, ["archetypes.db"], "archetype", (), search_index);
        render_and_index!(Action, ["actions.db"], "action", (), search_index);
        render_and_index!(Condition, ["conditionitems.db"], "condition", (), search_index);
        render_and_index!(Deity, ["deities.db"], "deity", (), search_index);
        let classfeatures = render_and_index!(ClassFeature, ["classfeatures.db"], "classfeature", &descriptions, search_index);
        render_and_index!(Class, ["classes.db"], "class", &classfeatures, search_index);
        render_and_index!(Equipment, ["equipment.db"], "item", &descriptions, search_index);
        render_and_index!(Ancestry, ["ancestries.db"], "ancestry", (), search_index);
        let bestiaries = bestiary_folders().expect("Could not read bestiary folders");
        render_and_index!(Npc, bestiaries, "creature", &descriptions, search_index);
    });
}

async fn build_search_index() -> Option<meilisearch_sdk::indexes::Index> {
    match std::env::var("MEILI_KEY") {
        Ok(key) => {
            let client = Client::new("http://localhost:7700", key);
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
            Some(search_index)
        }
        Err(_) => {
            println!("Indexing disabled. To publish data to meilisearch, please set MEILI_KEY in your environment");
            None
        }
    }
}

fn bestiary_folders() -> io::Result<Vec<String>> {
    Ok(fs::read_dir(&format!("{}/packs/data/", get_data_path()))?
        .filter_map(|f| f.ok())
        .filter(|f| f.path().is_dir())
        .map(|d| d.file_name().to_string_lossy().to_string())
        .filter(|d| d.contains("bestiary"))
        .filter(|d| !d.contains("ability"))
        .filter(|d| !d.contains("effects"))
        .filter(|d| !d.contains("april-fools")) // too many special cases to be worth it
        .inspect(|d| println!("Found bestiary folder {}", d))
        .collect())
}

fn text_cleanup(text: &str, remove_styling: bool) -> String {
    let resolved_references = REFERENCE_REGEX.replace_all(text, |caps: &Captures| {
        // These are compendium items only used for automation in foundry,
        // so they don’t contain meaningful links.
        if caps[1].ends_with("-effects") || &caps[1] == "pf2e-macros" {
            if caps[2].starts_with("Effect:") {
                String::new()
            } else {
                caps[2]
                    .strip_prefix("Spell Effect: ")
                    .map(|e| format!("[{}]", e))
                    .unwrap_or_else(|| caps[2].to_string())
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
    let cleaned_effects = &APPLIED_EFFECTS_REGEX.replace_all(replaced_references, "");
    let replaced_saves = &INLINE_SAVES_REGEX.replace_all(cleaned_effects, |caps: &Captures| format!("DC {} {}", &caps[1], &caps[2]));
    let done = replaced_saves.replace("<p>; ", "<p>");
    if remove_styling {
        INLINE_STYLE_REGEX.replace_all(&done, "").to_string()
    } else {
        done
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{creature::Creature, traits::TraitDescriptions};

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

    // change the path here to debug individual failing creatures
    #[test]
    fn _________edge_case_test() {
        match serde_json::from_str::<Creature>(&read_test_file("extinction-curse-bestiary.db/herecite-of-zevgavizeb.json")) {
            Ok(_) => (),
            Err(e) => panic!("Failed: {:?}", e),
        }
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

    #[test]
    fn effect_removal_test() {
        let input = "<p><strong>Frequency</strong> once per day</p>
<p><strong>Effect</strong> You gain a +10-foot status bonus to Speed for 1 minute.</p>
<p></p>
<hr />
<p>Automatically applied effects:</p>
<ul>
<li>+1 item bonus to Acrobatics checks.</li>
</ul>";
        assert_eq_ignore_linebreaks(
            &APPLIED_EFFECTS_REGEX.replace_all(input, ""),
            "<p><strong>Frequency</strong> once per day</p>
            <p><strong>Effect</strong> You gain a +10-foot status bonus to Speed for 1 minute.</p>
            <p></p>",
        );
    }

    #[test]
    fn spell_effect_replacement_test() {
        let input = "<li>
<strong>@Compendium[pf2e.spell-effects.Spell Effect: Animal Form (Ape)]{Ape}</strong>
<ul>
<li>Speed 25 feet, climb Speed 20 feet;</li>
<li><strong>Melee</strong> <span class=\"pf2-icon\">a</span> fist, <strong>Damage</strong> 2d6 bludgeoning.</li>
</ul>
</li>
<li><strong>@Compendium[pf2e.spell-effects.Spell Effect: Animal Form (Bear)]{Bear}</strong>
<ul>
<li>Speed 30 feet; </li><li><strong>Melee</strong> <span class=\"pf2-icon\">a</span> jaws, <strong>Damage</strong> 2d8 piercing;</li>
<li><strong>Melee</strong> <span class=\"pf2-icon\">a</span> claw (agile), <strong>Damage</strong> 1d8 slashing.</li>
</ul>
</li>";
        assert_eq!(text_cleanup(input, true), "<li>
<strong>[Animal Form (Ape)]</strong>
<ul>
<li>Speed 25 feet, climb Speed 20 feet;</li>
<li><strong>Melee</strong>  <img alt=\"One Action\" class=\"actionimage\" src=\"/static/actions/OneAction.webp\"> fist, <strong>Damage</strong> 2d6 bludgeoning.</li>
</ul>
</li>
<li><strong>[Animal Form (Bear)]</strong>
<ul>
<li>Speed 30 feet; </li><li><strong>Melee</strong>  <img alt=\"One Action\" class=\"actionimage\" src=\"/static/actions/OneAction.webp\"> jaws, <strong>Damage</strong> 2d8 piercing;</li>
<li><strong>Melee</strong>  <img alt=\"One Action\" class=\"actionimage\" src=\"/static/actions/OneAction.webp\"> claw (agile), <strong>Damage</strong> 1d8 slashing.</li>
</ul>
</li>");
    }

    #[test]
    fn inline_check_test() {
        let input = r#"<p>The dragon breathes a blast of flame that deals 20d6 fire damage in a 60-foot cone (<span data-pf2-check="reflex" data-pf2-dc="42" data-pf2-traits="arcane,evocation,fire,damaging-effect" data-pf2-label="Breath Weapon DC" data-pf2-show-dc="gm">basic Reflex</span> save)"#;
        assert_eq!(
            text_cleanup(input, true),
            "<p>The dragon breathes a blast of flame that deals 20d6 fire damage in a 60-foot cone (DC 42 basic Reflex save)"
        );

        let input = r#"<p>A Greater Disrupting weapon pulses with positive energy, dealing an extra 2d6 positive damage to undead On a critical hit, instead of being enfeebled 1, the undead creature must attempt a <span data-pf2-check="fortitude" data-pf2-dc="31 " data-pf2-label="Greater Disrupting DC" data-pf2-show-dc="GM">Fortitude</span> save with the following effects."#;
        assert_eq!(
            text_cleanup(input, true),
            "<p>A Greater Disrupting weapon pulses with positive energy, dealing an extra 2d6 positive damage to undead On a critical hit, instead of being enfeebled 1, the undead creature must attempt a DC 31 Fortitude save with the following effects."
        );
    }

    pub fn assert_eq_ignore_linebreaks(actual: &str, expected: &str) {
        assert_eq!(
            expected.lines().map(|l| l.trim()).collect::<String>(),
            actual.lines().map(|l| l.trim()).collect::<String>()
        );
    }
}
