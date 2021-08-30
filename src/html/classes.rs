use itertools::Itertools;

use super::Template;
use crate::data::class_features::ClassFeature;
use crate::data::{classes::Class, HasName};
use crate::html::Page;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};

const MAX_LEVEL: i32 = 20;

/*
 * pub name: String,
 * pub boost_levels: Vec<i32>,
 * pub ancestry_feat_levels: Vec<i32>,
 * pub attacks: AttacksProficiencies,
 * pub class_dc: Proficiency,
 * pub class_feat_levels: Vec<i32>,
 * pub defenses: DefensiveProficiencies,
 * pub description: String,
 * pub general_feat_levels: Vec<i32>,
 * pub hp: i32,
 * pub key_ability: Vec<AbilityScore>,
 * pub perception: Proficiency,
 * pub saving_throws: SavingThrowProficiencies,
 * pub skill_feat_levels: Vec<i32>,
 * pub skill_increase_levels: Vec<i32>,
 * pub trained_skills: Vec<Skill>,
 * pub free_skills: i32,
 * pub traits: Traits,
 * pub class_features: Vec<ClassItem>,
 */
impl Template<&[(ClassFeature, Page)]> for Class {
    fn render(&self, features: &[(ClassFeature, Page)]) -> Cow<'_, str> {
        let features_by_name: HashMap<_, _> = features.iter().map(|(f, p)| (f.name().to_owned(), (f, p))).collect();
        let mut page = String::with_capacity(10_000);
        page.push_str(&format!("<h1><a href=\"/class/{}\">{}</a></h1>", self.url_name(), self.name()));
        page.push_str("<table class=\"overview\">");
        page.push_str("<thead><tr><td>Level</td><td>Features</td></tr></thead>");
        let features_by_level = {
            let mut fbl = BTreeMap::new();
            self.class_features.iter().map(|f| features_by_name[&f.name]).for_each(|(f, p)| {
                fbl.entry(f.level).or_insert_with(Vec::new).push((f, p));
            });
            fbl
        };
        for level in 1..=MAX_LEVEL {
            page.push_str(&format!("<td>{}</td><td>", level));
            let mut features = Vec::new();
            if self.boost_levels.contains(&level) {
                features.push("Ability Boost");
            }
            if self.ancestry_feat_levels.contains(&level) {
                features.push("Ancestry Feat");
            }
            if self.class_feat_levels.contains(&level) {
                features.push("Class Feat");
            }
            if self.general_feat_levels.contains(&level) {
                features.push("General Feat");
            }
            if self.skill_feat_levels.contains(&level) {
                features.push("Skill Feat");
            }
            if self.skill_increase_levels.contains(&level) {
                features.push("Skill Increase");
            }
            let other_features = features_by_level
                .get(&level)
                .map(|fs| {
                    fs.iter()
                        .map(|&(f, _)| format!("<a href=\"/classfeature/{}\">{}</a>", f.url_name(), remove_level(f.name()),))
                        .join(", ")
                })
                .unwrap_or_default();
            if !other_features.is_empty() {
                features.push(&other_features);
            }
            page.push_str(&features.join(", "));
            page.push_str("</td></tr>");
        }
        page.push_str("</table>");
        for (_, p) in (1..=MAX_LEVEL).filter_map(|l| features_by_level.get(&l)).flatten() {
            page.push_str(p.content.split("<h2>Traits</h2>").next().unwrap_or(&p.content));
        }
        // page.push_str(&self.description);
        Cow::Owned(page)
    }

    fn render_index(elements: &[(Self, Page)]) -> String {
        let mut page = String::with_capacity(1000);
        page.push_str("<h1>Classes</h1><hr/><div id=\"list\">");
        for (class, _) in elements {
            page.push_str(&format!("<h2><a href=\"/class/{}\">{}</a></h2><br/>", class.url_name(), class.name));
        }
        page.push_str("</div>");
        page
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Class")
    }
}

fn remove_level(s: &str) -> &str {
    s.split(" (").next().unwrap_or(s)
}
