use crate::data::traits::read_trait_descriptions;
use crate::{data::feats::Feat, html::feats::FeatTemplate};
use askama::Template;
use itertools::Itertools;
use std::io::{BufReader, BufWriter, Write};

mod data;
mod html;

fn main() {
    let data_path = std::env::args().nth(1).expect("Expected path to foundry packs/data directory");
    let descriptions = read_trait_descriptions("tests/data/en.json");
    let list = std::fs::read_dir(&data_path)
        .expect("Could not find specified data directory")
        .map(|f| {
            let filename = f.expect("Could not read file").path();
            // println!("Reading {}", filename.to_str().unwrap());
            let f = std::fs::File::open(&filename).expect("File missing");
            let reader = BufReader::new(f);

            let feat: Feat = serde_json::from_reader(reader).expect("Deserialization failed");

            // maybe add a regex dependency at some point? not sure if we need it yet
            let safe_name = feat
                .name
                .to_lowercase()
                .replace(' ', "_")
                .replace('\'', "")
                .replace('(', "_")
                .replace(')', "_");
            let output_file = format!("{}.html", safe_name);
            let f2 = std::fs::File::create(&format!("output/{}", output_file)).expect("Could not create output file");
            let feat = FeatTemplate::new(feat, &descriptions);
            let mut writer = BufWriter::new(f2);
            write!(writer, "{}", feat.render().expect("Failed to render")).expect("Failed to write");
            (output_file, feat.name)
        })
        .map(|(filename, name)| format!(r#"<li><a href="{}">{}</a></li>"#, filename, name))
        .join("\n");
    println!("{}", list);
}
