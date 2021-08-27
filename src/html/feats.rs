use super::{Page, Template};
use crate::{
    data::{feats::Feat, traits::TraitDescriptions, HasName},
    html::{inline_rarity_if_not_common, render_trait_legend, render_traits},
};
use itertools::Itertools;
use lazy_static::lazy_static;
use meilisearch_sdk::document::Document;
use std::{borrow::Cow, fs, io};

// TODO: automate getting these
const CLASSES: &[&str] = &[
    "Alchemist",
    "Barbarian",
    "Bard",
    "Champion",
    "Cleric",
    "Druid",
    "Fighter",
    "Investigator",
    // "Magus"
    "Monk",
    "Oracle",
    "Ranger",
    "Rogue",
    "Sorcerer",
    // "Summoner"
    "Swashbuckler",
    "Witch",
    "Wizard",
];

const SKILLS: &[&str] = &[
    "Acrobatics",
    "Arcana",
    "Athletics",
    "Crafting",
    "Deception",
    "Diplomacy",
    "Intimidation",
    "Medicine",
    "Nature",
    "Occultism",
    "Performance",
    "Religion",
    "Society",
    "Stealth",
    "Survival",
    "Thievery",
];

const ANCESTRIES: &[&str] = &[
    "Anadi",
    "Android",
    "Azarketi",
    "Catfolk",
    "Conrasu",
    "Dwarf",
    "Elf",
    "Fetchling",
    "Fleshwarp",
    "Gnoll",
    "Gnome",
    "Goblin",
    "Goloma",
    "Grippli",
    "Halfling",
    "Hobgoblin",
    "Human",
    "Kitsune",
    "Kobold",
    "Leshy",
    "Lizardfolk",
    "Orc",
    "Ratfolk",
    "Shisk",
    "Shoony",
    "Sprite",
    "Strix",
    "Tengu",
];

impl Template<&TraitDescriptions> for Feat {
    fn render(&self, trait_descriptions: &TraitDescriptions) -> Cow<'_, str> {
        let mut page = String::with_capacity(5000);
        page.push_str(&format!(
            "<h1><a href=\"/feat/{}\">{}</a> {}<span class=\"type\">Feat {}</span></h1><hr/>",
            self.url_name(),
            &self.name,
            self.action_type.img(&self.actions),
            if self.level != 0 {
                self.level.to_string()
            } else {
                String::from("(Automatic)")
            },
        ));
        render_traits(&mut page, &self.traits);
        if !self.prerequisites.is_empty() {
            page.push_str("<b>Prerequisites</b> ");
            page.push_str(&self.prerequisites.join(", "));
            page.push_str("<hr/>");
        }
        page.push_str(&self.description);
        page.push_str("<hr/>");
        render_trait_legend(&mut page, &self.traits, trait_descriptions);
        Cow::Owned(page)
    }

    fn render_index(elements: &[(Self, Page)]) -> String {
        let feats = elements.iter().filter(|(f, _)| f.level != 0).collect_vec();
        render_full_feat_list(&feats)
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Feat")
    }

    fn render_subindices(target: &str, elements: &[(Self, Page)]) -> io::Result<()> {
        let feats = elements.iter().filter(|(f, _)| f.level != 0).collect_vec();
        for class in CLASSES {
            fs::write(
                &format!("{}/{}_index", target, class.to_lowercase()),
                render_filtered_feat_list(&feats, class),
            )?
        }
        for ancestry in ANCESTRIES {
            fs::write(
                &format!("{}/{}_index", target, ancestry.to_lowercase()),
                render_filtered_feat_list(&feats, ancestry),
            )?
        }
        for skill in SKILLS {
            fs::write(
                &format!("{}/{}_index", target, skill.to_lowercase()),
                render_skill_feat_list(&feats, skill),
            )?
        }
        fs::write(&format!("{}/general_index", target), render_general_feat_list(&feats))?;
        Ok(())
    }
}

fn render_full_feat_list(feats: &[&(Feat, Page)]) -> String {
    let mut page = render_feat_list_header(None);
    page.push_str("<table class=\"overview\">");
    page.push_str("<thead><tr><td>Name</td><td>Level</td></tr></thead>");
    for (feat, _) in feats {
        page.push_str(&format!(
            "<tr><td><a href=\"{}\">{} {}</a><td>{}</td></tr>",
            feat.url_name(),
            feat.name,
            feat.action_type.img(&feat.actions),
            feat.level,
        ));
    }
    page.push_str("</table>");
    page
}

fn render_feat_row(feat: &Feat, page: &Page) -> String {
    format!(
        r#"
<div class="pseudotr">
<input id="cl-{}" class="toggle" type="checkbox">
<label for="cl-{}" class="lt">{} {} {}<span class="lvl">{}</span></label>
<div class="cpc">{}</div>
</input>
</div>
"#,
        page.get_uid(),
        page.get_uid(),
        feat.name(),
        feat.action_type.img(&feat.actions),
        inline_rarity_if_not_common(&feat.traits.rarity),
        feat.level,
        &page.content
    )
}

fn render_filtered_feat_list(feats: &[&(Feat, Page)], filter_trait: &str) -> String {
    let mut page = render_feat_list_header(Some(filter_trait));
    let trait_lower = filter_trait.to_lowercase();
    for (feat, p) in feats.iter().filter(|(f, _)| f.traits.value.contains(&trait_lower)) {
        page.push_str(&render_feat_row(feat, p));
    }
    page
}

fn render_general_feat_list(feats: &[&(Feat, Page)]) -> String {
    let page = render_feat_list_header(Some("General"));
    feats
        .iter()
        .filter(|(f, _)| f.traits.value.contains(&GENERAL_TRAIT))
        .filter(|(f, _)| !f.traits.value.contains(&SKILL_TRAIT))
        .fold(page, |mut page, (feat, p)| {
            page.push_str(&render_feat_row(feat, p));
            page
        })
}

fn render_skill_feat_list(feats: &[&(Feat, Page)], skill: &str) -> String {
    let page = render_feat_list_header(Some(skill));
    feats
        .iter()
        .filter(|(f, _)| f.traits.value.contains(&SKILL_TRAIT))
        .filter(|(f, _)| !f.traits.value.contains(&ARCHETYPE_TRAIT))
        .filter(|(f, _)| f.prerequisites.iter().any(|p| p.contains(skill)))
        .fold(page, |mut page, (feat, p)| {
            page.push_str(&render_feat_row(feat, p));
            page
        })
}

lazy_static! {
    static ref STATIC_FEAT_HEADER: String = {
        let mut header = String::with_capacity(3000);
        fn collapsible_toc(header: &mut String, list: &[&str], list_name: &str) {
            header.push_str(&format!(
                r#"
<input id="cl-{}list" class="toggle" type="checkbox"/>
<label for="cl-{}list" class="lt exp-header">Filter by {}</span></label>
<div class="cpc header fw">
"#,
                list_name, list_name, list_name,
            ));
            if list_name == "Skill" {
                header.push_str(r#"<span><a href="general_index"><div>General</div></a></span>"#);
            }
            for e in list {
                header.push_str(&format!(
                    r#"<span><a href="{}_index"><div>{}</div></a></span>"#,
                    e.to_lowercase(),
                    e
                ));
            }
            header.push_str("</div></input>");
        }
        collapsible_toc(&mut header, CLASSES, "Class");
        collapsible_toc(&mut header, SKILLS, "Skill");
        collapsible_toc(&mut header, ANCESTRIES, "Ancestry");
        header
    };
    static ref SKILL_TRAIT: String = String::from("skill");
    static ref GENERAL_TRAIT: String = String::from("general");
    static ref ARCHETYPE_TRAIT: String = String::from("archetype");
}

fn render_feat_list_header(category: Option<&str>) -> String {
    let mut page = String::with_capacity(50_000);
    page.push_str(&STATIC_FEAT_HEADER);
    match category {
        Some(c) => {
            page.push_str("<h1>");
            page.push_str(c);
            page.push_str(" Feats</h1><hr/>");
        }
        None => page.push_str("<h1>Feats</h1><hr/>"),
    }
    page
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{read_test_file, DESCRIPTIONS};

    #[test]
    fn test_feat_template() {
        let feat: Feat = serde_json::from_str(&read_test_file("feats.db/sever-space.json")).expect("Deserialization failed");
        let expected = include_str!("../../tests/html/sever_space.html");
        assert_eq!(
            feat.render(&DESCRIPTIONS).lines().collect::<String>(),
            expected.lines().collect::<String>()
        );
    }
}
