use super::Template;
use crate::data::ability_scores::AbilityScore;
use crate::data::class_features::ClassFeature;
use crate::data::classes::{AttackProficiencies, ClassItem, DefensiveProficiencies};
use crate::data::proficiency::Proficiency;
use crate::data::{classes::Class, HasName};
use crate::html::Page;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};

const MAX_LEVEL: i32 = 20;

lazy_static! {
    static ref CHOICE_CLASS_SKILLS_REGEX: regex::Regex = regex::Regex::new("Trained in your choice of [\\w ]+").unwrap();
}

/*
 * pub name: String,
 * pub description: String,
 * pub traits: Traits,
 */
impl Template<&[(ClassFeature, Page)]> for Class {
    fn render(&self, features: &[(ClassFeature, Page)]) -> Cow<'_, str> {
        let mut page = String::with_capacity(10_000);
        page.push_str(&format!("<h1><a href=\"/class/{}\">{}</a></h1><hr/>", self.url_name(), self.name()));

        add_hp(self.hp, &mut page);
        add_key_ability(&self.key_ability, &mut page);
        add_proficiencies(self, &mut page);

        page.push_str("<h2>Class Features</h2><hr/>");
        let features_by_level = group_features_by_level(&self.class_features, features);
        add_feature_table(self, &features_by_level, &mut page);
        for (_, p) in (1..=MAX_LEVEL).filter_map(|l| features_by_level.get(&l)).flatten() {
            page.push_str(p.content.split("<h2>Traits</h2>").next().unwrap_or(&p.content));
        }
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

fn add_hp(hp: i32, page: &mut String) {
    page.push_str("<h3>Hit Points</h3>");
    page.push_str(&format!(
        "<p>At first level and every level thereafter, you increase your maximum hit points by {} plus your constitution modifier.",
        hp
    ));
}

fn add_key_ability(key_abilities: &[AbilityScore], page: &mut String) {
    page.push_str("<h3>Key Ability</h3>");
    page.push_str("<p>");
    page.push_str("At first level, you increase one of these scores by 2. Subclasses (such as the rogue’s rackets) might offer additional options.<br/>");
    page.push_str("<b>Key Ability: </b>");
    page.push_str(&key_abilities.iter().map_into::<&str>().join(" or "));
    page.push_str("</p>");
}

fn add_proficiencies(class: &Class, page: &mut String) {
    page.push_str("<h2>Initial Proficiencies</h2><hr/>");
    add_saves(class, page);
    add_offenses(&class.attacks, &class.name, &class.class_dc, page);
    add_defenses(&class.defenses, page);
    add_skills(class, page);
}

fn add_saves(class: &Class, page: &mut String) {
    page.push_str("<h3>Saves</h3>");
    page.push_str("<p>");
    page.push_str(class.perception.as_ref());
    page.push_str(" in Perception<br/>");
    page.push_str(class.saving_throws.reflex.as_ref());
    page.push_str(" in Reflex Saves<br/>");
    page.push_str(class.saving_throws.will.as_ref());
    page.push_str(" in Will Saves<br/>");
    page.push_str(class.saving_throws.fortitude.as_ref());
    page.push_str(" in Fortitude Saves<br/>");
    page.push_str("</p>");
}

// To get nicer formatting, we use the fact that
// (1) classes start with the same proficiency in all armor they’re at least trained in
// (2) training in heavier armor implies training in lighter armor
// (3) all classes are at least trained in unarmored
fn add_defenses(defenses: &DefensiveProficiencies, page: &mut String) {
    page.push_str("<h3>Armor</h3>");
    page.push_str("<p>");
    match defenses {
        DefensiveProficiencies {
            unarmored: p,
            light: Proficiency::Untrained,
            medium: _,
            heavy: _,
        } => page.push_str(&format!("{} in unarmored<br/>Untrained in all armor<br/>", p.as_ref())),
        DefensiveProficiencies {
            unarmored: p,
            light: p2,
            medium: Proficiency::Untrained,
            heavy: _,
        } if p == p2 => page.push_str(&format!(
            "{} in unarmored and light armor<br/>Untrained in medium and heavy armor<br/>",
            p.as_ref()
        )),
        DefensiveProficiencies {
            unarmored: p,
            light: p2,
            medium: p3,
            heavy: Proficiency::Untrained,
        } if p == p2 && p2 == p3 => page.push_str(&format!(
            "{} in unarmored, light, and medium armor<br/>Untrained in heavy armor<br/>",
            p.as_ref()
        )),
        DefensiveProficiencies {
            unarmored: p,
            light: p2,
            medium: p3,
            heavy: p4,
        } if p == p2 && p2 == p3 && p3 == p4 => page.push_str(&format!("{} in all armor<br/>", p.as_ref())),
        _ => unimplemented!("Unimplemented armor proficiencies: {:?}", defenses),
    }
    page.push_str("</p>");
}

fn add_offenses(offenses: &AttackProficiencies, name: &str, class_dc: &Proficiency, page: &mut String) {
    page.push_str("<h3>Weapons</h3>");
    page.push_str("<p>");
    page.push_str(&format!("{} in unarmed<br/>", offenses.unarmed.as_ref()));
    if offenses.simple != Proficiency::Untrained {
        page.push_str(&format!("{} in simple weapons<br/>", offenses.simple.as_ref()));
    }
    if offenses.martial != Proficiency::Untrained {
        page.push_str(&format!("{} in martial weapons<br/>", offenses.martial.as_ref()));
    }
    if offenses.advanced != Proficiency::Untrained {
        page.push_str(&format!("{} in advanced weapons<br/>", offenses.advanced.as_ref()));
    }
    if !offenses.other.name.is_empty() {
        page.push_str(&format!("{} in {}<br/>", offenses.other.rank.as_ref(), &offenses.other.name));
    }
    if class_dc != &Proficiency::Untrained {
        page.push_str(&format!("{} in {} class DC<br/>", class_dc.as_ref(), name));
    }
    page.push_str("</p>");
}

fn add_skills(class: &Class, page: &mut String) {
    page.push_str("<h3>Skills</h3>");
    page.push_str("<p>");
    match class.trained_skills.as_slice() {
        [] => (),
        [skill] => page.push_str(&format!("Trained in {}<br/>", skill.as_ref())),
        [s1, s2] => page.push_str(&format!("Trained in {} and {}<br/>", s1.as_ref(), s2.as_ref())),
        // “Trained in Acrobatics, Athletics, Arcana, and Intimidation”
        [all_but_last @ .., last] => page.push_str(&format!(
            "Trained in {}, and {}<br/>",
            all_but_last.iter().map_into::<&str>().join(", "),
            last.as_ref(),
        )),
    }
    if let Some(choice) = CHOICE_CLASS_SKILLS_REGEX.find(&class.description) {
        page.push_str(choice.as_str());
        page.push_str("<br/>");
    }
    page.push_str(&format!(
        "Trained in a number of skills equal to {} plus your intelligence modifier<br/>",
        class.free_skills
    ));
    page.push_str("</p>");
}
fn group_features_by_level<'a>(
    features: &[ClassItem],
    all_features: &'a [(ClassFeature, Page)],
) -> BTreeMap<i32, Vec<(&'a ClassFeature, &'a Page)>> {
    let features_by_name: HashMap<_, _> = all_features.iter().map(|(f, p)| (f.name().to_owned(), (f, p))).collect();
    let mut fbl = BTreeMap::new();
    features
        .iter()
        .map(|f| {
            *features_by_name
                .get(f.name.trim_start_matches("(Choice) "))
                .unwrap_or_else(|| panic!("Classfeature {} not found", &f.name))
        })
        .for_each(|(f, p)| {
            fbl.entry(f.level).or_insert_with(Vec::new).push((f, p));
        });
    fbl
}

fn add_feature_table(class: &Class, features_by_level: &BTreeMap<i32, Vec<(&ClassFeature, &Page)>>, page: &mut String) {
    page.push_str("<table class=\"overview\">");
    page.push_str("<thead><tr><td>Level</td><td>Features</td></tr></thead>");
    for level in 1..=MAX_LEVEL {
        page.push_str(&format!("<td>{}</td><td>", level));
        let mut features = Vec::new();
        if class.boost_levels.contains(&level) {
            features.push("Ability Boost");
        }
        if class.ancestry_feat_levels.contains(&level) {
            features.push("Ancestry Feat");
        }
        if class.class_feat_levels.contains(&level) {
            features.push("Class Feat");
        }
        if class.general_feat_levels.contains(&level) {
            features.push("General Feat");
        }
        if class.skill_feat_levels.contains(&level) {
            features.push("Skill Feat");
        }
        if class.skill_increase_levels.contains(&level) {
            features.push("Skill Increase");
        }
        let other_features = features_by_level
            .get(&level)
            .map(|fs| {
                fs.iter()
                    .map(|&(f, _)| format!("<a href=\"/classfeature/{}\">{}</a>", f.url_name(), f.without_variant()))
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{data::classes::OtherAttacksProficiencies, tests::read_test_file};

    #[test]
    fn offenses_test() {
        let mut s = String::new();
        add_offenses(
            &AttackProficiencies {
                unarmed: Proficiency::Trained,
                simple: Proficiency::Expert,
                martial: Proficiency::Master,
                advanced: Proficiency::Legendary,
                other: OtherAttacksProficiencies {
                    name: String::from("RAW"),
                    rank: Proficiency::Trained,
                },
            },
            "Gamemaster",
            &Proficiency::Untrained,
            &mut s,
        );
        assert_eq!("<h3>Weapons</h3><p>Trained in unarmed<br/>Expert in simple weapons<br/>Master in martial weapons<br/>Legendary in advanced weapons<br/>Trained in RAW<br/></p>", s);
    }

    #[test]
    fn defenses_test() {
        let mut s = String::new();
        add_defenses(
            &DefensiveProficiencies {
                unarmored: Proficiency::Expert,
                light: Proficiency::Expert,
                medium: Proficiency::Untrained,
                heavy: Proficiency::Untrained,
            },
            &mut s,
        );
        assert_eq!(
            "<h3>Armor</h3><p>Expert in unarmored and light armor<br/>Untrained in medium and heavy armor<br/></p>",
            s
        );
    }

    #[test]
    fn skill_test() {
        let mut s = String::new();
        let fighter: Class = serde_json::from_str(&read_test_file("classes.db/fighter.json")).expect("Deserialization failed");
        add_skills(&fighter, &mut s);
        assert_eq!("<h3>Skills</h3><p>Trained in your choice of Acrobatics or Athletics<br/>Trained in a number of skills equal to 3 plus your intelligence modifier<br/></p>", s);
    }
}
