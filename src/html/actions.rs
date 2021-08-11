use crate::data::action_type::ActionType;
use crate::data::actions::Action;
use crate::data::traits::{Rarity, Trait, TraitDescriptions};
use askama::Template;
use convert_case::{Case, Casing};

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
