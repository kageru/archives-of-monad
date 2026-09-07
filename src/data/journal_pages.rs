use serde::Deserialize;
use std::{collections::HashMap, fs};

// Descriptive lore that used to be embedded directly in an ancestry/class/feat/etc.'s own
// description is now often split out into a journal entry page instead, with the item's
// description just holding a short blurb plus an `@UUID[...JournalEntryPage...]` reference to the
// full text. We read every journal page up front and key it by id so `text_cleanup` in
// `parser.rs` can inline the real content instead of just a bare link/label.
#[derive(Deserialize)]
struct JsonJournal {
    #[serde(default)]
    pages: Vec<JsonJournalPage>,
}

#[derive(Deserialize)]
struct JsonJournalPage {
    #[serde(rename = "_id")]
    id: String,
    text: Option<JsonJournalText>,
}

#[derive(Deserialize)]
struct JsonJournalText {
    content: String,
}

pub fn read_journal_pages(path: &str) -> HashMap<String, String> {
    let dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Could not read journals folder {path}: {e}");
            return HashMap::new();
        }
    };
    dir.filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .filter_map(|path| fs::read_to_string(&path).ok().map(|content| (path, content)))
        .filter_map(|(path, content)| match serde_json::from_str::<JsonJournal>(&content) {
            Ok(journal) => Some(journal),
            Err(e) => {
                eprintln!("Could not parse journal {path:?}: {e}");
                None
            }
        })
        .flat_map(|journal| journal.pages)
        .filter_map(|page| page.text.map(|text| (page.id, text.content)))
        .collect()
}
