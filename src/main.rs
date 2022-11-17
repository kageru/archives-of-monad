// While this lint is nice in theory, write! can return errors that won’t realistically happen and don’t need to be handled in this project.
#![allow(unused_must_use)]
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
};
use futures::executor::block_on;
use html::render;
use itertools::Itertools;
use lazy_static::lazy_static;
use meilisearch_sdk::client::*;
pub use parser::text_cleanup;
use regex::Regex;
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

    static ref URL_REPLACEMENTS: Regex = Regex::new(r"[^A-Za-z0-9]").unwrap();
    // Things to strip from short description. We can’t just remove all tags because we at least
    // want to keep <a> and probably <em>/<b>
    static ref HTML_FORMATTING_TAGS: Regex = Regex::new("</?(p|br|hr|div|span|h1|h2|h3)[^>]*>").unwrap();
}

static FAILED_COMPENDIA: AtomicI32 = AtomicI32::new(0);

fn get_data_path() -> &'static str {
    &DATA_PATH
}

macro_rules! render_and_index {
    ($type: ty, $source: expr, $target: literal, $additional: expr, $index: ident) => {
        match render::<$type, _, _>(&$source, concat!("output/", $target), $additional) {
            Ok(rendered) => {
                if let Some(index) = &$index {
                    if let Err(e) = index
                        .add_or_replace(&rendered.iter().cloned().map(|(_, page)| page).collect_vec(), Some("id"))
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
        render_and_index!(Action, ["actions.db", "adventure-specific-actions.db"], "action", (), search_index);
        render_and_index!(Condition, ["conditions.db"], "condition", (), search_index);
        render_and_index!(Deity, ["deities.db"], "deity", (), search_index);
        let classfeatures = render_and_index!(ClassFeature, ["classfeatures.db"], "classfeature", &TRANSLATIONS, search_index);
        render_and_index!(Class, ["classes.db"], "class", &classfeatures, search_index);
        render_and_index!(Equipment, ["equipment.db"], "item", &TRANSLATIONS, search_index);
        render_and_index!(
            AncestryFeature,
            ["ancestryfeatures.db"],
            "ancestryfeature",
            &TRANSLATIONS,
            search_index
        );
        render_and_index!(Ancestry, ["ancestries.db"], "ancestry", (), search_index);
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
    Ok(fs::read_dir(format!("{}/packs/data/", get_data_path()))?
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
        match serde_json::from_str::<Creature>(&read_test_file("pfs-season-3-bestiary.db/explosive-rat.json")) {
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
