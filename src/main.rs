#[macro_use]
extern crate enum_display_derive;
use crate::data::backgrounds::Background;
use crate::data::conditions::Condition;
use crate::data::deities::Deity;
use crate::data::traits::read_trait_descriptions;
use crate::html::spells::SpellTemplate;
use crate::{data::traits::TraitDescriptions, html::feats::FeatTemplate};
use askama::Template;
use data::HasName;
use serde::Deserialize;
use std::io::BufReader;
use std::{fs, io};

mod data;
mod html;

static mut DATA_PATH: String = String::new();

fn get_data_path() -> &'static str {
    unsafe { &DATA_PATH }
}

fn main() {
    unsafe {
        DATA_PATH = std::env::args().nth(1).expect("Expected path to foundry module root");
    }
    let descriptions = read_trait_descriptions(&format!("{}/static/lang/en.json", get_data_path()));
    match render_category("feats.db", "output/feats", &descriptions, FeatTemplate::new) {
        Ok(_) => println!("Successfully rendered feats"),
        Err(e) => eprintln!("Error while rendering feats: {}", e),
    }
    match render_category("spells.db", "output/spells", &descriptions, SpellTemplate::new) {
        Ok(_) => println!("Successfully rendered spells"),
        Err(e) => eprintln!("Error while rendering spells: {}", e),
    }
    match render_category("deities.db", "output/deities", &descriptions, |deity: Deity, _| deity) {
        Ok(_) => println!("Successfully rendered deities"),
        Err(e) => eprintln!("Error while rendering deities: {}", e),
    }
    match render_category("backgrounds.db", "output/backgrounds", &descriptions, |bg: Background, _| bg) {
        Ok(_) => println!("Successfully rendered backgounds"),
        Err(e) => eprintln!("Error while rendering backgounds: {}", e),
    }
    match render_category(
        "conditionitems.db",
        "output/conditions",
        &descriptions,
        |condition: Condition, _| condition,
    ) {
        Ok(_) => println!("Successfully rendered conditions"),
        Err(e) => eprintln!("Error while rendering conditions: {}", e),
    }
}

fn render_category<T: for<'de> Deserialize<'de> + HasName, R: Template, F: FnMut(T, &TraitDescriptions) -> R>(
    src_path: &str,
    output_path: &str,
    descriptions: &TraitDescriptions,
    mut convert: F,
) -> io::Result<()> {
    fs::create_dir_all(output_path)?;
    let mut list = String::with_capacity(100_000);
    list.push_str("<ul>");
    for f in fs::read_dir(&format!("{}/packs/data/{}", get_data_path(), src_path))? {
        let filename = f?.path();
        //println!("Reading {}", filename.to_str().unwrap());
        let f = fs::File::open(&filename)?;
        let reader = BufReader::new(f);
        let object: T = serde_json::from_reader(reader).expect("Deserialization failed");
        let name = object.name().to_owned();
        let output_filename = format!("{}.html", object.url_name());
        let full_output_filename = &format!("{}/{}", output_path, output_filename);
        let template = convert(object, descriptions);
        fs::write(full_output_filename, template.render().expect("Failed to render"))?;
        list.push_str(&format!("<li><a href=\"{}\">{}</a></li>\n", output_filename, name));
    }
    list.push_str("</ul>");
    list.push_str("<div style=\"height: 2em\"></div>");
    list.push_str("<a href=\"../index.html\">Back</a>");
    fs::write(&format!("{}/index.html", output_path), &list)?;
    Ok(())
}
