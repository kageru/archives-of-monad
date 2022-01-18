#[macro_use]
extern crate strum;
use crate::data::creature::Npc;
use data::{
    actions::Action,
    ancestries::Ancestry,
    ancestry_features::AncestryFeature,
    archetypes::Archetype,
    backgrounds::Background,
    class_features::ClassFeature,
    classes::Class,
    conditions::Condition,
    deities::Deity,
    equipment::Equipment,
    feats::Feat,
    heritages::Heritage,
    spells::Spell,
    traits::{read_translations, render_traits, Translations},
    HasName, ObjectName,
};
use futures::executor::block_on;
use html::render;
use itertools::Itertools;
use lazy_static::lazy_static;
use meilisearch_sdk::client::*;
pub use parser::text_cleanup;
use regex::{Captures, Regex};
use std::{
    fs, io,
    sync::atomic::{AtomicI32, Ordering},
};

mod data;
mod html;
mod parser;

lazy_static! {
    static ref DATA_PATH: String = std::env::args().nth(1).unwrap_or_else(|| String::from("foundry"));

    static ref TRANSLATIONS: Translations = read_translations(
        &format!("{}/static/lang/en.json", get_data_path()),
        &[&format!("{}/static/lang/re-en.json", get_data_path())],
    );

    static ref REFERENCE_REGEX: Regex = Regex::new(r"@Compendium\[pf2e\.(.*?)\.(.*?)\]\{(.*?)}").unwrap();
    static ref LEGACY_INLINE_ROLLS: Regex = Regex::new(r"\[\[/b?r ([^#\]]+(?: #[\w ]+)?)\]\](\{(?:.*?)})?").unwrap();
    static ref INLINE_ROLLS: Regex = Regex::new(r"\[\[/b?r \{[^}]*\}\[[^\]]*\]\]\]\{([^}]*)}").unwrap();
    static ref URL_REPLACEMENTS: Regex = Regex::new(r"[^A-Za-z0-9]").unwrap();
    // Things to strip from short description. We can’t just remove all tags because we at least
    // want to keep <a> and probably <em>/<b>
    static ref HTML_FORMATTING_TAGS: Regex = Regex::new("</?(p|br|hr|div|span|h1|h2|h3)[^>]*>").unwrap();
    static ref ACTION_GLYPH_REGEX: Regex = Regex::new("<span class=\"pf2-icon\">([ADTFRadtfr123/]+)</span>").unwrap();
    static ref INLINE_STYLE_REGEX: Regex = Regex::new(r#" style="[^"]+""#).unwrap();
    static ref APPLIED_EFFECTS_REGEX: Regex = Regex::new("(<hr ?/>\n?)?<p>Automatically applied effects:</p>\n?<ul>(.|\n)*</ul>").unwrap();
    static ref LOCALIZATION_REGEX: Regex = Regex::new("@Localize\\[(.*?)\\]").unwrap();
    static ref TEMPLATE_REGEX: Regex = Regex::new(r"@Template\[type:(\w+)\|distance:(\d+)\](\{[^}]*\})?").unwrap();
}

static FAILED_COMPENDIA: AtomicI32 = AtomicI32::new(0);

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
                eprintln!(concat!("Error while rendering ", $target, " folder : {}"), e);
                FAILED_COMPENDIA.fetch_add(1, Ordering::SeqCst);
                vec![]
            }
        }
    };
}

fn main() {
    block_on(async move {
        let search_index = build_search_index().await;

        match (render_traits("output/trait", &TRANSLATIONS), &search_index) {
            (Ok(traits), Some(index)) => {
                index.add_or_replace(&traits, None).await.unwrap();
            }
            (Ok(_), None) => println!("Successfully rendered descriptions"),
            (Err(e), _) => eprintln!("Error while rendering descriptions: {}", e),
        }

        render_and_index!(Feat, ["feats.db"], "feat", &TRANSLATIONS, search_index);
        render_and_index!(Spell, ["spells.db"], "spell", &TRANSLATIONS, search_index);
        render_and_index!(Background, ["backgrounds.db"], "background", (), search_index);
        render_and_index!(Archetype, ["archetypes.db"], "archetype", (), search_index);
        render_and_index!(Action, ["actions.db"], "action", (), search_index);
        render_and_index!(Condition, ["conditionitems.db"], "condition", (), search_index);
        render_and_index!(Deity, ["deities.db"], "deity", (), search_index);
        let classfeatures = render_and_index!(ClassFeature, ["classfeatures.db"], "classfeature", &TRANSLATIONS, search_index);
        render_and_index!(Class, ["classes.db"], "class", &classfeatures, search_index);
        render_and_index!(Equipment, ["equipment.db"], "item", &TRANSLATIONS, search_index);
        let ancestryfeatures = render_and_index!(
            AncestryFeature,
            ["ancestryfeatures.db"],
            "ancestryfeature",
            &TRANSLATIONS,
            search_index
        );
        render_and_index!(Ancestry, ["ancestries.db"], "ancestry", &ancestryfeatures, search_index);
        render_and_index!(Heritage, ["heritages.db"], "heritage", (), search_index);
        let bestiaries = bestiary_folders().expect("Could not read bestiary folders");
        render_and_index!(Npc, bestiaries, "creature", &TRANSLATIONS, search_index);
    });
    std::process::exit(FAILED_COMPENDIA.load(Ordering::SeqCst)); // nonzero return if anything failed
}

async fn build_search_index() -> Option<meilisearch_sdk::indexes::Index> {
    match std::env::var("MEILI_KEY") {
        Ok(key) => {
            let client = Client::new("http://localhost:7700", key);
            let search_index = client.index("all");
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

fn text_cleanup_old(text: &str) -> String {
    let localized = LOCALIZATION_REGEX.replace_all(text, |caps: &Captures| {
        TRANSLATIONS
            .get_by_key(&caps[1])
            .unwrap_or_else(|| panic!("No translation found for {}", &caps[1]))
    });
    let resolved_references = REFERENCE_REGEX.replace_all(&localized, |caps: &Captures| {
        // These are compendium items only used for automation in foundry,
        // so they don’t contain meaningful links.
        // Bestiary abilities are an open TODO.
        if caps[1].ends_with("-effects") || &caps[1] == "pf2e-macros" || &caps[1] == "bestiary-family-ability-glossary" {
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
                "vehicles" => "creature",
                "heritages" => "heritage",
                c => unimplemented!("{}", c),
            };
            let element = ObjectName(&caps[2]);
            format!(r#"<a href="/{}/{}">{}</a>"#, category, element.url_name(), &caps[3])
        }
    });
    let clean_rolls = &INLINE_ROLLS.replace_all(&resolved_references, |caps: &Captures| caps[1].to_string());
    let resolved_icons = LEGACY_INLINE_ROLLS.replace_all(clean_rolls, |caps: &Captures| {
        caps.get(2)
            .map(|m| m.as_str().trim_matches(|c| c == '{' || c == '}').to_string())
            .unwrap_or_else(|| caps[1].replace('#', ""))
    });
    let replaced_references = &ACTION_GLYPH_REGEX.replace_all(&resolved_icons, |caps: &Captures| {
        let mut replacement = String::from(" ");
        replacement.push_str(get_action_img(&caps[1]).unwrap_or(""));
        replacement
    });
    let cleaned_effects = &APPLIED_EFFECTS_REGEX.replace_all(replaced_references, "");
    let templates = &TEMPLATE_REGEX.replace_all(cleaned_effects, |caps: &Captures| format!("{}-foot {}", &caps[2], &caps[1]));
    let no_empty = templates.replace("<p>; ", "<p>");
    INLINE_STYLE_REGEX.replace_all(&no_empty, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{creature::Creature, traits::Translations};
    use pretty_assertions::assert_eq;

    pub fn read_test_file(path: &str) -> String {
        fs::read_to_string(format!("foundry/packs/data/{}", path)).expect("Could not find file")
    }
    lazy_static! {
        pub static ref TRANSLATIONS: Translations = read_translations("foundry/static/lang/en.json", &["foundry/static/lang/re-en.json"]);
    }

    // change the path here to debug individual failing creatures
    #[test]
    fn _________edge_case_test() {
        match serde_json::from_str::<Creature>(&read_test_file("pfs-season-3-bestiary.db/aydrian-thrune-3-4.json")) {
            Ok(_) => (),
            Err(e) => panic!("Failed: {:?}", e),
        }
    }

    pub fn assert_eq_ignore_linebreaks(actual: &str, expected: &str) {
        assert_eq!(
            expected.lines().map(|l| l.trim()).collect::<String>(),
            actual.lines().map(|l| l.trim()).collect::<String>()
        );
    }
}
