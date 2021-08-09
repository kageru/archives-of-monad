#[macro_use]
extern crate enum_display_derive;
use crate::data::archetypes::Archetype;
use crate::data::backgrounds::Background;
use crate::data::conditions::Condition;
use crate::data::deities::Deity;
use crate::data::traits::read_trait_descriptions;
use crate::data::ObjectName;
use crate::html::feats::FeatTemplate;
use crate::html::spells::SpellTemplate;
use askama::Template;
use data::HasName;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::BufReader;
use std::{fs, io};

mod data;
mod html;

lazy_static! {
    static ref DATA_PATH: String = std::env::args().nth(1).expect("Expected path to foundry module root");
    static ref REFERENCE_REGEX: Regex = Regex::new(r"@Compendium\[pf2e\.(.*?)\.(.*?)\]\{(.*?)}").unwrap();
    static ref REFERENCE_URLS: HashMap<&'static str, &'static str> = HashMap::from([
        ("feats-srd", "feats"),
        ("conditionitems", "conditions"),
        ("spells-srd", "spells"),
        ("actionspf2e", "actions"),
        ("equipment-srd", "equipment"),
        // unsure, maybe these should just both be features?
        ("ancestryfeatures", "ancestryfeatures"),
        ("classfeatures", "classfeatures"),
        ("pathfinder-bestiary", "creatures"),
        ("archetypes", "archetypes"),
        ("backgrounds", "backgrounds"),
    ]);
}

fn get_data_path() -> &'static str {
    &DATA_PATH
}

fn main() {
    let descriptions = read_trait_descriptions(&format!("{}/static/lang/en.json", get_data_path()));
    // let mut items_for_replacement: HashMap<&str, HashMap<String, Box<dyn HasName>>> = HashMap::new();
    match render_category("feats.db", "output/feats", &descriptions, FeatTemplate::new) {
        Ok(_) => println!("Successfully rendered feats"),
        Err(e) => eprintln!("Error while rendering feats: {}", e),
    }
    // items_for_replacement.insert("feats-srd", feats_by_name);
    match render_category("spells.db", "output/spells", &descriptions, SpellTemplate::new) {
        Ok(_) => println!("Successfully rendered spells"),
        Err(e) => eprintln!("Error while rendering spells: {}", e),
    }
    match render_category("deities.db", "output/deities", &descriptions, |deity: Deity, _| deity) {
        Ok(_) => println!("Successfully rendered deities"),
        Err(e) => eprintln!("Error while rendering deities: {}", e),
    }
    match render_category("backgrounds.db", "output/backgrounds", &(), |bg: Background, _| Background {
        description: replace_references(&bg.description).to_string(),
        ..bg
    }) {
        Ok(_) => println!("Successfully rendered backgounds"),
        Err(e) => eprintln!("Error while rendering backgounds: {}", e),
    }
    match render_category("conditionitems.db", "output/conditions", &descriptions, |c: Condition, _| c) {
        Ok(_) => println!("Successfully rendered conditions"),
        Err(e) => eprintln!("Error while rendering conditions: {}", e),
    }
    match render_category("archetypes.db", "output/archetypes", &(), |at: Archetype, _| Archetype {
        content: replace_references(&at.content).to_string(),
        ..at
    }) {
        Ok(_) => println!("Successfully rendered archetypes"),
        Err(e) => eprintln!("Error while rendering archetypes: {}", e),
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
        println!("Reading {}", filename.to_str().unwrap());
        let f = fs::File::open(&filename)?;
        let reader = BufReader::new(f);
        let object: T = serde_json::from_reader(reader).expect("Deserialization failed");
        let template = convert(object.clone(), additional_data);
        let output_filename = format!("{}", object.url_name());
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

fn replace_references<'a>(text: &'a str) -> Cow<'a, str> {
    REFERENCE_REGEX.replace_all(text, |caps: &Captures| {
        let category = REFERENCE_URLS[&caps[1]];
        let element = ObjectName(&caps[2]);
        format!(r#"<a href="/{}/{}">{}</a>"#, category, element.url_name(), &caps[3])
    })
}
