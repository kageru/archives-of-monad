use crate::data::damage::{Damage, DamageScaling, DamageType};
use crate::data::spells::{Area, Spell, SpellCategory, SpellComponents, SpellSchool, SpellTradition, SpellType};
use crate::data::traits::{Rarity, Trait, TraitDescriptions};
use askama::Template;
use convert_case::{Case, Casing};

#[derive(Template, PartialEq, Debug)]
#[template(path = "spell.html", escape = "none")]
pub struct SpellTemplate {
    pub name: String,
    pub area: Area,
    pub area_string: Option<String>, // not happy with this
    pub components: SpellComponents,
    pub cost: String,
    pub category: SpellCategory,
    pub damage: Damage,
    pub damage_type: DamageType,
    pub description: String,
    pub duration: String,
    pub level: i32,
    pub range: String,
    pub scaling: DamageScaling,
    pub school: SpellSchool,
    pub secondary_casters: i32,
    pub secondary_check: Option<String>,
    pub spell_type: SpellType,
    pub sustained: bool,
    pub target: String,
    pub time: String,
    pub traditions: Vec<SpellTradition>,
    pub traits: Vec<Trait>,
    pub rarity: Option<(Rarity, String)>,
}

impl SpellTemplate {
    pub fn new(spell: Spell, trait_descriptions: &TraitDescriptions) -> Self {
        let spell_category = if spell.traits.value.iter().any(|t| t == "cantrip") {
            SpellCategory::Cantrip
        } else {
            spell.category
        };

        let test = spell
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

        SpellTemplate {
            name: spell.name,
            area: spell.area,
            area_string: spell.area_string,
            components: spell.components,
            cost: spell.cost,
            category: spell_category,
            damage: spell.damage,
            damage_type: spell.damage_type,
            description: spell.description,
            duration: spell.duration,
            level: spell.level,
            range: spell.range,
            scaling: spell.scaling,
            school: spell.school,
            secondary_casters: spell.secondary_casters,
            secondary_check: spell.secondary_check,
            spell_type: spell.spell_type,
            sustained: spell.sustained,
            target: spell.target,
            time: spell.time,
            traditions: spell.traditions,
            traits: test,
            rarity: spell.traits.rarity.map(|r| (r, trait_descriptions.0[&r.to_string()].clone())),
        }
    }
}
