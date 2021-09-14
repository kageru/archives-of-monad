use super::Template;
use crate::{
    data::{creature::Creature, traits::TraitDescriptions, HasName},
    html::{render_trait_legend, render_traits, render_traits_inline},
};
use convert_case::{Case, Casing};
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
        page.push_str("<h1>Creatures</h1><hr><br/>");
        page.push_str("<table class=\"overview\">");
        page.push_str("<thead><tr><td>Name</td><td class=\"traitcolumn\">Traits</td><td>Source</td><td>Level</td></tr></thead>");
        for (creature, _) in elements {
            page.push_str(&format!(
                "<tr><td><a href=\"{}\">{}</a></td><td class=\"traitcolumn\">",
                creature.url_name(),
                creature.name,
            ));
            render_traits_inline(&mut page, &creature.traits);
            page.push_str(&format!("</td><td>{}</td><td>{}</td></tr>", creature.source, creature.level));
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
<b>AC</b> {}{}; <b>Fort</b> {}{}; <b>Reflex</b> {}{}; <b>Will</b> {}{}{}<br/>
<b>HP</b> {}{}<br/>
<b>Speed</b> {}{}<br/>
",
        creature.source,
        sig(creature.perception),
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
        creature
            .skills
            .iter()
            .map(|(skill, modifier)| format!("{} +{}", skill.as_ref(), modifier))
            .join(", "),
        sig(creature.ability_scores.strength),
        creature.ability_scores.strength,
        sig(creature.ability_scores.dexterity),
        creature.ability_scores.dexterity,
        sig(creature.ability_scores.constitution),
        creature.ability_scores.constitution,
        sig(creature.ability_scores.intelligence),
        creature.ability_scores.intelligence,
        sig(creature.ability_scores.wisdom),
        creature.ability_scores.wisdom,
        sig(creature.ability_scores.charisma),
        creature.ability_scores.charisma,
        creature.ac,
        if let Some(details) = &creature.ac_details {
            format!(" {}", details)
        } else {
            String::new()
        },
        sig(creature.saves.fortitude),
        creature.saves.fortitude,
        sig(creature.saves.reflex),
        creature.saves.reflex,
        sig(creature.saves.will),
        creature.saves.will,
        if let Some(m) = &creature.saves.additional_save_modifier {
            format!("; {}", m)
        } else {
            String::new()
        },
        creature.hp,
        match &creature.hp_details {
            Some(details) if !details.is_empty() => format!(" ({})", details),
            _ => String::new(),
        },
        creature.speeds.value,
        if !creature.speeds.other_speeds.is_empty() {
            format!(
                " ({})",
                creature
                    .speeds
                    .other_speeds
                    .iter()
                    .map(|speed| format!("<b>{}</b> {}", speed.speed_type, speed.value))
                    .join(", ")
            )
        } else {
            String::new()
        },
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
    for attack in &creature.attacks {
        let (first, second, third) = calculate_maps(attack.modifier, &attack.traits.misc);
        page.push_str(&format!(
            "<b>{}</b> +{} ({}{}, {}{}) to hit ",
            attack.name,
            first,
            sig(second),
            second,
            sig(third),
            third
        ));
        if !attack.traits.misc.is_empty() {}
        page.push('(');
        page.push_str(
            &attack
                .traits
                .misc
                .iter()
                .map(|s| s.from_case(Case::Kebab).to_case(Case::Lower))
                .join(", "),
        );
        page.push_str(") ");
        page.push_str(
            &attack
                .damage
                .iter()
                .map(|dmg| format!("{} {}", dmg.damage, dmg.damage_type.as_ref()))
                .join(" + "),
        );
        page.push_str("<br/>");
    }
    page.push_str("<hr/>");
    page.push_str(&creature.flavor_text);
    page.push_str("<hr/>");
    render_trait_legend(&mut page, &creature.traits, descriptions);
    page
}

fn sig(i: i32) -> &'static str {
    if i >= 0 {
        "+"
    } else {
        ""
    }
}

fn calculate_maps(modifier: i32, traits: &[String]) -> (i32, i32, i32) {
    let map = if traits.iter().any(|t| t == "agile" || t == "Agile") {
        4
    } else {
        5
    };
    (modifier, modifier - map, modifier - 2 * map)
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
