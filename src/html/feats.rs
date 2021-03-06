use crate::{
    data::{feats::Feat, traits::Translations, HasName},
    html::{inline_rarity_if_not_common, render_trait_legend, render_traits, write_full_html_document, HtmlPage, Template},
};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::{borrow::Cow, fmt::Write, io};

// TODO: automate getting these
// For now, run
// ls -1 | sed -E 's/([a-z])([a-z]+)\.json/"\U\1\L\2",/'
// in the respective directories.
const CLASSES: &[&str] = &[
    "Alchemist",
    "Barbarian",
    "Bard",
    "Champion",
    "Cleric",
    "Druid",
    "Fighter",
    "Gunslinger",
    "Inventor",
    "Investigator",
    "Magus",
    "Monk",
    "Oracle",
    "Psychic",
    "Ranger",
    "Rogue",
    "Sorcerer",
    "Summoner",
    "Swashbuckler",
    "Thaumaturge",
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
    "Lore",
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
    "Aasimar",
    "Anadi",
    "Android",
    "Aphorite",
    "Automaton",
    "Azarketi",
    "Beastkin",
    "Catfolk",
    "Changeling",
    "Conrasu",
    "Dhampir",
    "Duskwalker",
    "Dwarf",
    "Elf",
    "Fetchling",
    "Fleshwarp",
    "Ganzi",
    "Gnoll",
    "Gnome",
    "Goblin",
    "Goloma",
    "Grippli",
    "Half-Elf",
    "Half-Orc",
    "Halfling",
    "Hobgoblin",
    "Human",
    "Ifrit",
    "Kitsune",
    "Kobold",
    "Leshy",
    "Lizardfolk",
    "Orc",
    "Oread",
    "Pixie",
    "Poppet",
    "Ratfolk",
    "Reflection",
    "Shisk",
    "Shoony",
    "Skeleton",
    "Sprite",
    "Strix",
    "Suli",
    "Sylph",
    "Tengu",
    "Tiefling",
    "Undine",
];

impl Template<&Translations> for Feat {
    fn render(&self, trait_descriptions: &Translations) -> Cow<'_, str> {
        let mut page = String::with_capacity(50000);
        render_single_feat(&mut page, trait_descriptions, self);
        Cow::Owned(page)
    }

    fn header(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(&STATIC_SELECTION_FEAT_HEADER))
    }

    fn render_index(elements: &[(Self, HtmlPage)]) -> String {
        let feats = elements.iter().filter(|(f, _)| f.level != 0).collect_vec();
        render_full_feat_list(&feats)
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Feat")
    }

    fn render_subindices(target: &str, elements: &[(Self, HtmlPage)]) -> io::Result<()> {
        let feats = elements.iter().filter(|(f, _)| f.level != 0).collect_vec();
        for class in CLASSES {
            write_full_html_document(
                &format!("{}/{}_index", target, class.to_lowercase()),
                &format!("{} Feats", class),
                &render_filtered_feat_list(&feats, class, FeatListType::Class),
            )?;
        }
        for ancestry in ANCESTRIES {
            write_full_html_document(
                &format!("{}/{}_index", target, ancestry.to_lowercase()),
                &format!("{} Feats", ancestry),
                &render_filtered_feat_list(&feats, ancestry, FeatListType::Ancestry),
            )?;
        }
        for skill in SKILLS {
            write_full_html_document(
                &format!("{}/{}_index", target, skill.to_lowercase()),
                &format!("{} Feats", skill),
                &render_skill_feat_list(&feats, skill),
            )?;
        }
        write_full_html_document(
            &format!("{}/general_index", target),
            "General Feats",
            &render_general_feat_list(&feats),
        )
    }
}

fn render_single_feat(page: &mut String, trait_descriptions: &Translations, feat: &Feat) {
    write!(
        page,
        "<h1><a href=\"/feat/{}\">{}</a> {}<span class=\"type\">Feat {}</span></h1><hr/>",
        feat.url_name(),
        &feat.name,
        feat.action_type.img(&feat.actions),
        if feat.level != 0 {
            feat.level.to_string()
        } else {
            String::from("(Automatic)")
        },
    );
    render_traits(page, &feat.traits);
    if !feat.source.is_empty() {
        write!(page, "<b>Source</b> {}<br/>", feat.source);
    }
    if !feat.prerequisites.is_empty() {
        page.push_str("<b>Prerequisites</b> ");
        page.push_str(&feat.prerequisites.join(", "));
    }
    if !feat.source.is_empty() || !feat.prerequisites.is_empty() {
        page.push_str("<hr/>");
    }
    page.push_str(&feat.description);
    page.push_str("<hr/>");
    render_trait_legend(page, &feat.traits, trait_descriptions);
}

fn render_full_feat_list(feats: &[&(Feat, HtmlPage)]) -> String {
    let mut page = render_feat_list_header(None, FeatListType::Unknown, None);
    page.push_str("<table class=\"overview\">");
    page.push_str("<thead><tr><td>Name</td><td>Level</td></tr></thead>");
    for (feat, _) in feats {
        write!(
            page,
            "<tr><td><a href=\"{}\">{} {}</a><td>{}</td></tr>",
            feat.url_name(),
            feat.name,
            feat.action_type.img(&feat.actions),
            feat.level,
        );
    }
    page.push_str("</table>");
    page
}

fn render_feat_row(s: &mut String, feat: &Feat, page: &HtmlPage) {
    write!(
        s,
        r#"
<div class="pseudotr">
<label for="cl-{}" class="lt">{} {} {}<span class="lvl">{}</span></label>
<input id="cl-{}" class="toggle" type="checkbox">
<div class="cpc">{}</div>
</div>
"#,
        page.id,
        feat.name(),
        feat.action_type.img(&feat.actions),
        inline_rarity_if_not_common(&feat.traits.rarity),
        feat.level,
        page.id,
        &page.content
    );
}

fn render_filtered_feat_list(feats: &[&(Feat, HtmlPage)], filter_trait: &str, list_type: FeatListType) -> String {
    let mut page = render_feat_list_header(Some(filter_trait), list_type, Some(filter_trait));
    let trait_lower = filter_trait.to_lowercase();
    for (feat, p) in feats.iter().filter(|(f, _)| f.traits.misc.contains(&trait_lower)) {
        render_feat_row(&mut page, feat, p);
    }
    page
}

fn render_general_feat_list(feats: &[&(Feat, HtmlPage)]) -> String {
    let mut page = render_feat_list_header(Some("General"), FeatListType::Unknown, None);
    feats
        .iter()
        .filter(|(f, _)| f.traits.misc.contains(&GENERAL_TRAIT))
        .filter(|(f, _)| !f.traits.misc.contains(&SKILL_TRAIT))
        .for_each(|(feat, p)| {
            render_feat_row(&mut page, feat, p);
        });
    page
}

fn render_skill_feat_list(feats: &[&(Feat, HtmlPage)], skill: &str) -> String {
    let skill_lower = skill.to_lowercase();
    let mut page = render_feat_list_header(Some(skill), FeatListType::Skill, Some(skill));
    feats
        .iter()
        .filter(|(f, _)| f.traits.misc.contains(&SKILL_TRAIT))
        .filter(|(f, _)| !f.traits.misc.contains(&ARCHETYPE_TRAIT))
        .filter(|(f, _)| f.prerequisites.iter().any(|p| p.to_lowercase().contains(&skill_lower)))
        .for_each(|(feat, p)| {
            render_feat_row(&mut page, feat, p);
        });
    page
}

#[derive(PartialEq)]
enum FeatListType {
    // General,
    Skill,
    Class,
    Ancestry,
    Unknown,
}

const HEADER_LABELS: &str = r#"
<div class="header fw">
<a href="/feat/general_index" class="hoverlink">General (No Skill)</a>
<label for="cl-Classlist" class="lt">Filter by Class</label>
<label for="cl-Skilllist" class="lt">Filter by Skill</label>
<label for="cl-Ancestrylist" class="lt">Filter by Ancestry</label>
</div>
"#;
const CLASS_FEAT_HEADER_LABELS: &str = r#"
<div class="header fw">
<a href="/feat/general_index" class="hoverlink">General (No Skill)</a>
<label for="cl-Classlist" class="lt pseudolink">Filter by Class</label>
<label for="cl-Skilllist" class="lt">Filter by Skill</label>
<label for="cl-Ancestrylist" class="lt">Filter by Ancestry</label>
</div>
"#;
const SKILL_FEAT_HEADER_LABELS: &str = r#"
<div class="header fw">
<a href="/feat/general_index" class="hoverlink">General (No Skill)</a>
<label for="cl-Classlist" class="lt">Filter by Class</label>
<label for="cl-Skilllist" class="lt pseudolink">Filter by Skill</label>
<label for="cl-Ancestrylist" class="lt">Filter by Ancestry</label>
</div>
"#;
const ANCESTRY_FEAT_HEADER_LABELS: &str = r#"
<div class="header fw">
<a href="/feat/general_index" class="hoverlink">General (No Skill)</a>
<label for="cl-Classlist" class="lt">Filter by Class</label>
<label for="cl-Skilllist" class="lt">Filter by Skill</label>
<label for="cl-Ancestrylist" class="lt pseudolink">Filter by Ancestry</label>
</div>
"#;
fn collapsible_toc(header: &mut String, list: &[&str], list_name: &str, expanded: bool, highlighted: Option<&str>) {
    write!(
        header,
        r#"
<input id="cl-{}list" class="toggle" type="radio" name="featheader"{}>
<div class="cpc header fw">
"#,
        list_name,
        if expanded { " checked" } else { "" }
    );
    for e in list {
        write!(
            header,
            r#"<a href="{}_index"{}>{} </a>"#,
            e.to_lowercase(),
            if expanded && Some(e) == highlighted.as_ref() {
                ""
            } else {
                " class=\"hoverlink\""
            },
            e
        );
    }
    header.push_str("</div>");
}

fn render_selection_header(header: &mut String, list_type: FeatListType, highlighted: Option<&str>) {
    header.push_str(match list_type {
        FeatListType::Skill => SKILL_FEAT_HEADER_LABELS,
        FeatListType::Class => CLASS_FEAT_HEADER_LABELS,
        FeatListType::Ancestry => ANCESTRY_FEAT_HEADER_LABELS,
        FeatListType::Unknown => HEADER_LABELS,
    });
    collapsible_toc(header, CLASSES, "Class", list_type == FeatListType::Class, highlighted);
    collapsible_toc(header, SKILLS, "Skill", list_type == FeatListType::Skill, highlighted);
    collapsible_toc(header, ANCESTRIES, "Ancestry", list_type == FeatListType::Ancestry, highlighted);
}

lazy_static! {
    // Static header with nothing highlighted or expanded
    static ref STATIC_SELECTION_FEAT_HEADER: String = {
        let mut header = String::with_capacity(3000);
        render_selection_header(&mut header, FeatListType::Unknown, None);
        header
    };
    static ref SKILL_TRAIT: String = String::from("skill");
    static ref GENERAL_TRAIT: String = String::from("general");
    static ref ARCHETYPE_TRAIT: String = String::from("archetype");
}

fn render_feat_list_header(category: Option<&str>, list_type: FeatListType, selection: Option<&str>) -> String {
    let mut page = String::with_capacity(50_000);
    if list_type == FeatListType::Unknown && selection.is_none() {
        page.push_str(&STATIC_SELECTION_FEAT_HEADER);
    } else {
        render_selection_header(&mut page, list_type, selection);
    }
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
    use crate::tests::{assert_eq_ignore_linebreaks, read_test_file, TRANSLATIONS};

    #[test]
    fn test_feat_template() {
        let feat: Feat = serde_json::from_str(&read_test_file("feats.db/sever-space.json")).expect("Deserialization failed");
        let mut s = String::new();
        render_single_feat(&mut s, &TRANSLATIONS, &feat);
        assert_eq_ignore_linebreaks(&s, include_str!("../../tests/html/sever_space.html"));
    }

    #[test]
    fn test_render_feat_header() {
        assert_eq_ignore_linebreaks(
            &STATIC_SELECTION_FEAT_HEADER,
            include_str!("../../tests/html/no_selection_feat_header.html"),
        );
    }
}
