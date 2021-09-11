use super::Template;
use crate::{
    data::{creature::Creature, traits::TraitDescriptions, HasName},
    html::render_traits,
};
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

fn render_creature(creature: &Creature, _descriptions: &TraitDescriptions) -> String {
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
<b>Perception</b> {}{}<br/>
<b>Skills</b> {}<br/>
<b>Str</b> {}{}, <b>Dex</b> {}{}, <b>Con</b> {}{}, <b>Int</b> {}{}, <b>Wis</b> {}{}, <b>Cha</b> {}{}<br/>
<hr/>",
        creature.source,
        if creature.perception > 0 { "+" } else { "" },
        creature.perception,
        "TODO",
        if creature.ability_scores.strength > 0 { "+" } else { "" },
        creature.ability_scores.strength,
        if creature.ability_scores.dexterity > 0 { "+" } else { "" },
        creature.ability_scores.dexterity,
        if creature.ability_scores.constitution > 0 { "+" } else { "" },
        creature.ability_scores.constitution,
        if creature.ability_scores.intelligence > 0 { "+" } else { "" },
        creature.ability_scores.intelligence,
        if creature.ability_scores.wisdom > 0 { "+" } else { "" },
        creature.ability_scores.wisdom,
        if creature.ability_scores.charisma > 0 { "+" } else { "" },
        creature.ability_scores.charisma,
    ));
    page
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
