use crate::data::traits::read_trait_descriptions;
use crate::{data::feats::Feat, html::feats::FeatTemplate};
use askama::Template;
use std::io::{BufReader, BufWriter, Write};

mod data;
mod html;

fn main() {
    let f = std::fs::File::open("tests/data/feats/sever-space.json").expect("File missing");
    let reader = BufReader::new(f);
    let feat: Feat = serde_json::from_reader(reader).expect("Deserialization failed");

    let descriptions = read_trait_descriptions("tests/data/en.json");

    let feat = FeatTemplate::new(feat, &descriptions);

    let f2 = std::fs::File::create("test.html").unwrap();
    let mut writer = BufWriter::new(f2);
    write!(writer, "{}", feat.render().unwrap()).unwrap();
}
