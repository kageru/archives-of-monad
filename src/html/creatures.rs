use super::Template;
use crate::{
    data::{creature::Creature, traits::TraitDescriptions, HasName},
    html::{render_trait_legend, render_traits},
};
use itertools::Itertools;
use std::borrow::Cow;

impl Template<&TraitDescriptions> for Creature {
    fn render(&self, descriptions: &TraitDescriptions) -> Cow<'_, str> {
        Cow::Owned(render_creature(self, descriptions))
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Creature")
    }

    fn render_index(elements: &[(Self, super::Page)]) -> String {
        let mut page = String::with_capacity(250_000);
        page.push_str("<h1>Creatures</h1><hr><br/><br/>");
        page.push_str("<table class=\"overview\">");
        page.push_str("<thead><tr><td>Name</td><td>Level</td></tr></thead>");
        for (creature, _) in elements {
            page.push_str(&format!(
                "<tr><td><a href=\"{}\">{}</a></td><td>{}</td></tr>",
                creature.url_name(),
                creature.name,
                creature.level,
            ));
        }
        page.push_str("</table>");
        page
    }
}

fn render_creature(creature: &Creature, descriptions: &TraitDescriptions) -> String {
    let mut page = String::with_capacity(20_000);
    page.push_str(&format!(
        "<h1><a href=\"/creature/{}\">{}</a><span class=\"type\">Creature {}</h1><hr/>",
        creature.url_name(),
        &creature.name,
        &creature.level
    ));
    render_traits(&mut page, &creature.traits);
    page.push_str(&format!(
        "
<b>Source</b> {}<br/>
<b>Perception</b> {}{}{}<br/>
<b>Languages</b> {}<br/>
<b>Skills</b> {}<br/>
<b>Str</b> {}{}, <b>Dex</b> {}{}, <b>Con</b> {}{}, <b>Int</b> {}{}, <b>Wis</b> {}{}, <b>Cha</b> {}{}<br/>
<hr/>
<b>AC</b> {}; <b>Fort</b> {}; <b>Reflex</b> {}; <b>Will</b> {}{}<br/>
<b>HP</b> {}<br/>
",
        creature.source,
        if creature.perception >= 0 { "+" } else { "" },
        creature.perception,
        if !creature.senses.is_empty() {
            format!(" ({})", creature.senses)
        } else {
            String::new()
        },
        if creature.languages.is_empty() {
            "none".to_string()
        } else {
            creature.languages.join(", ")
        },
        "TODO",
        if creature.ability_scores.strength >= 0 { "+" } else { "" },
        creature.ability_scores.strength,
        if creature.ability_scores.dexterity >= 0 { "+" } else { "" },
        creature.ability_scores.dexterity,
        if creature.ability_scores.constitution >= 0 { "+" } else { "" },
        creature.ability_scores.constitution,
        if creature.ability_scores.intelligence >= 0 { "+" } else { "" },
        creature.ability_scores.intelligence,
        if creature.ability_scores.wisdom >= 0 { "+" } else { "" },
        creature.ability_scores.wisdom,
        if creature.ability_scores.charisma >= 0 { "+" } else { "" },
        creature.ability_scores.charisma,
        creature.ac,
        creature.saves.fortitude,
        creature.saves.reflex,
        creature.saves.will,
        if let Some(m) = &creature.saves.additional_save_modifier {
            format!("; {}", m)
        } else {
            String::new()
        },
        creature.hp,
    ));
    if !creature.immunities.is_empty() {
        page.push_str(&format!("<b>Immunities</b> {}<br/>", creature.immunities.join(", ")));
    }
    if !creature.weaknesses.is_empty() {
        page.push_str(&format!("<b>Weaknesses</b> {}<br/>", format_resistance(&creature.weaknesses)));
    }
    if !creature.resistances.is_empty() {
        page.push_str(&format!("<b>Resistances</b> {}<br/>", format_resistance(&creature.resistances)));
    }
    page.push_str("<hr/>");
    page.push_str(&creature.flavor_text);
    page.push_str("<hr/>");
    render_trait_legend(&mut page, &creature.traits, descriptions);
    page
}

fn format_resistance(xs: &[(String, Option<i32>)]) -> String {
    xs.iter()
        .map(|(label, value)| {
            if let Some(v) = value {
                format!("{} {}", label, v)
            } else {
                label.to_string()
            }
        })
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{read_test_file, DESCRIPTIONS};

    #[test]
    fn test_render_budget_dahak() {
        let dargon: Creature =
            serde_json::from_str(&read_test_file("pathfinder-bestiary.db/ancient-red-dragon.json")).expect("Deserialization failed");
        let dargon: String = render_creature(&dargon, &DESCRIPTIONS).lines().collect();
        let expected = include_str!("../../tests/html/budget_dahak.html");
        assert_eq!(dargon, expected.lines().collect::<String>());
    }
}
