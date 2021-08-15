use crate::{
    data::{
        action_type::ActionType,
        feat_type::FeatType,
        feats::Feat,
        traits::{Rarity, Trait, TraitDescriptions},
    },
    replace_references,
};
use askama::Template;
use convert_case::{Case, Casing};

#[derive(Template, PartialEq, Debug)]
#[template(path = "feat.html", escape = "none")]
pub struct FeatTemplate {
    pub name: String,
    pub action_type: ActionType,
    pub actions: Option<i32>,
    pub description: String,
    pub feat_type: FeatType,
    pub level: i32,
    pub prerequisites: Vec<String>,
    pub traits: Vec<Trait>,
    pub rarity: Option<(Rarity, String)>,
}

impl FeatTemplate {
    pub fn new(feat: Feat, trait_descriptions: &TraitDescriptions) -> Self {
        let traits = feat
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

        FeatTemplate {
            name: feat.name,
            action_type: feat.action_type,
            actions: feat.actions,
            description: replace_references(&feat.description),
            feat_type: feat.feat_type,
            level: feat.level,
            prerequisites: feat.prerequisites,
            traits,
            rarity: feat.traits.rarity.map(|r| (r, trait_descriptions.0[&r.to_string()].clone())),
        }
    }
}
