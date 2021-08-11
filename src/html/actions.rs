use crate::data::action_type::ActionType;
use crate::data::actions::Action;
use crate::data::traits::{Rarity, Trait, TraitDescriptions};
use crate::data::HasName;
use crate::get_data_path;
use askama::Template;
use convert_case::{Case, Casing};
use itertools::Itertools;
use std::{fs, io, io::BufReader};

#[derive(Template, PartialEq, Debug)]
#[template(path = "action.html", escape = "none")]
pub struct ActionTemplate {
    pub name: String,
    pub description: String,
    pub action_type: ActionType,
    pub number_of_actions: Option<i32>,
    pub traits: Vec<Trait>,
    pub rarity: Option<(Rarity, String)>,
}

impl ActionTemplate {
    pub fn new(action: Action, trait_descriptions: &TraitDescriptions) -> Self {
        let test = action
            .traits
            .value
            .iter()
            .map(|name| name.to_case(Case::Pascal))
            .map(|name| Trait {
                description: trait_descriptions
                    .0
                    .get(&name)
                    .cloned()
                    .unwrap_or_else(|| String::from("NOT_FOUND")),
                name,
            })
            .collect();

        ActionTemplate {
            name: action.name,
            description: action.description,
            action_type: action.action_type,
            number_of_actions: action.number_of_actions,
            traits: test,
            rarity: action.traits.rarity.map(|r| (r, trait_descriptions.0[&r.to_string()].clone())),
        }
    }
}

pub fn render_action_list(folder: &str, target: &str) -> io::Result<()> {
    let mut all_actions = fs::read_dir(&format!("{}/packs/data/{}", get_data_path(), folder))?
        .filter_map(|f| {
            let filename = f.ok()?.path();
            // println!("Reading {}", filename.to_str().unwrap());
            let f = fs::File::open(&filename).ok()?;
            let reader = BufReader::new(f);
            let action: Action = serde_json::from_reader(reader).expect("Deserialization failed");
            Some(action)
        })
        .collect_vec();
    // Sort first by name and then by level. Don’t use unstable sorting here!
    all_actions.sort_by_key(|s| s.name.clone());
    let mut page = String::with_capacity(10_000);
    page.push_str("<div id=\"gridlist\">");
    for action in &all_actions {
        page.push_str(&format!(
            "<span><a href=\"{}\">{} {}</a></span>",
            action.url_name(),
            action.name(),
            action.action_type.img(&action.number_of_actions)
        ));
    }
    fs::write(format!("{}/index.html", target), page)
}
