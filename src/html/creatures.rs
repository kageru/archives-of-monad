use super::Template;
use crate::{
    data::{
        creature::{Attack, Creature, Npc, OtherCreatureSpeed, SpellCasting},
        damage::CreatureDamage,
        spells::Spell,
        traits::TraitDescriptions,
        HasLevel, HasName,
    },
    get_action_img,
    html::{render_trait_legend, render_traits, render_traits_inline, spells::spell_level_as_string, write_full_page},
};
use convert_case::{Case, Casing};
use itertools::Itertools;
use std::{
    borrow::{Borrow, Cow},
    collections::HashMap,
    fmt::{self, Display},
};

impl Template<&TraitDescriptions> for Npc {
    fn render(&self, descriptions: &TraitDescriptions) -> Cow<'_, str> {
        if let Npc::Creature(c) = &self {
            Cow::Owned(render_creature(c, descriptions))
        } else {
            Cow::Borrowed("")
        }
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Creature")
    }

    fn render_index(elements: &[(Self, super::Page)]) -> String {
        let mut page = String::with_capacity(250_000);
        page.push_str("<h1>Creatures</h1><hr><br/>");
        fill_index(
            &mut page,
            &elements
                .iter()
                .filter_map(|(n, _)| match n {
                    Npc::Creature(c) => Some(c.borrow()),
                    _ => None,
                })
                .collect_vec(),
        );
        page
    }

    fn render_subindices(target: &str, elements: &[(Self, super::Page)]) -> std::io::Result<()> {
        let mut by_trait = HashMap::new();
        for (c, _) in elements {
            if let Npc::Creature(creature) = c {
                for t in &creature.traits.misc {
                    by_trait.entry(t).or_insert_with(Vec::new).push(creature.borrow());
                }
            }
        }
        for (t, cs) in by_trait {
            let mut page = String::with_capacity(250_000);
            page.push_str(&format!("<h1>{} Creatures</h1><hr><br/>", t));
            fill_index(&mut page, &cs);
            write_full_page(
                &format!("{}/trait_{}", target, t.to_lowercase()),
                &format!("{} Creatures", t),
                &page,
            )?;
        }
        Ok(())
    }
}

fn fill_index(page: &mut String, elements: &[&Creature]) {
    page.push_str("<table class=\"overview\">");
    page.push_str("<thead><tr><td>Name</td><td class=\"traitcolumn\">Traits</td><td>Source</td><td>Level</td></tr></thead>");
    for creature in elements {
        page.push_str(&format!(
            "<tr><td><a href=\"{}\">{}</a></td><td class=\"traitcolumn\">",
            creature.url_name(),
            creature.name(),
        ));
        render_traits_inline(page, &creature.traits);
        page.push_str(&format!("</td><td>{}</td><td>{}</td></tr>", creature.source, creature.level));
    }
    page.push_str("</table>");
}

fn render_creature(creature: &Creature, descriptions: &TraitDescriptions) -> String {
    let mut page = String::with_capacity(20_000);
    page.push_str(&format!(
        "<h1><a href=\"/creature/{}\">{}</a><span class=\"type\">Creature {}</span></h1><hr/>",
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
        other_speeds(&creature.speeds.other_speeds),
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
    render_attacks(&creature.attacks, &mut page);
    for spellcasting in &creature.spellcasting {
        render_spells(spellcasting, &mut page, creature.level());
    }
    if !creature.spellcasting.is_empty() {
        page.push_str("<hr/>")
    }
    if let Some(flavor_text) = &creature.flavor_text {
        page.push_str(flavor_text);
        page.push_str("<hr/>");
    }
    render_trait_legend(&mut page, &creature.traits, descriptions);
    page
}

fn other_speeds(other_speeds: &[OtherCreatureSpeed]) -> String {
    let format_speed = |speed: &OtherCreatureSpeed| format!("<b>{}</b> {}", speed.speed_type, speed.value);
    if !other_speeds.is_empty() {
        format!(" ({})", other_speeds.iter().map(format_speed).join(", "))
    } else {
        String::new()
    }
}

fn render_spells(casting: &SpellCasting, page: &mut String, creature_level: i32) {
    page.push_str(&if casting.casting_type.has_dc() {
        format!("<b>{} (DC {})</b><br/><p>", casting.name, casting.dc)
    } else {
        format!("<b>{}</b><br/><p>", casting.name)
    });
    let cantrip_level = ((creature_level + 1) / 2).clamp(1, 10);
    for (level, spells) in &casting.spells.iter().group_by(|s| s.level()) {
        if level == 0 {
            page.push_str(&format!("<b>Cantrips ({}):</b> ", spell_level_as_string(cantrip_level)));
        } else {
            page.push_str(&format!(
                "<b>{}{}:</b> ",
                spell_level_as_string(level),
                slots_for_level(casting, level)
            ));
        }
        page.push_str(
            &spells
                .into_iter()
                .map(|s| PreparedSpell(s, 1))
                .coalesce(|s1, s2| (s1.0.name == s2.0.name).then(|| PreparedSpell(s1.0, s1.1 + s2.1)).ok_or((s1, s2)))
                .map(|s| s.to_string())
                .join(", "),
        );
        page.push_str("<br/>");
    }
    page.push_str("</p>");
}

struct PreparedSpell<'a>(&'a Spell, i32);

impl Display for PreparedSpell<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.1 > 1 {
            write!(f, "<a href=\"/spell/{}\">{} ({}x)</a>", &self.0.url_name(), &self.0.name(), self.1)
        } else {
            write!(f, "<a href=\"/spell/{}\">{}</a>", &self.0.url_name(), &self.0.name())
        }
    }
}

fn slots_for_level(casting: &SpellCasting, level: i32) -> String {
    let has_slots = casting.casting_type.has_slots();
    casting
        .slots
        .get(&level)
        .filter(|&&n| n != 0)
        .filter(|_| has_slots)
        .map(|n| format!(" ({} slots)", n))
        .unwrap_or_else(String::new)
}

fn render_attacks(attacks: &[Attack], page: &mut String) {
    if attacks.is_empty() {
        return;
    }
    for attack in attacks {
        add_to_hit_and_maps(attack, page);
        add_attack_traits(attack, page);
        add_attack_damage(page, attack);
    }
    page.push_str("<hr/>");
}

fn add_attack_damage(page: &mut String, attack: &Attack) {
    let format_dmg = |dmg: &CreatureDamage| format!("{} {}", dmg.damage, dmg.damage_type.as_ref());
    page.push_str(&attack.damage.iter().map(format_dmg).join(" + "));
    page.push_str("<br/>");
}

fn kebap_to_lower(s: &str) -> String {
    s.from_case(Case::Kebab).to_case(Case::Lower)
}

fn add_attack_traits(attack: &Attack, page: &mut String) {
    if !attack.traits.misc.is_empty() {
        page.push('(');
        page.push_str(&attack.traits.misc.iter().map(|s| kebap_to_lower(s)).join(", "));
        page.push_str(") ");
    }
}

fn add_to_hit_and_maps(attack: &Attack, page: &mut String) {
    let (first, second, third) = calculate_maps(attack.modifier, &attack.traits.misc);
    page.push_str(&format!(
        "<b>{}</b> {} +{} ({}{}, {}{}) to hit ",
        attack.name,
        get_action_img("1").unwrap(),
        first,
        sig(second),
        second,
        sig(third),
        third
    ));
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
    use crate::{
        data::{
            creature::Npc,
            damage::DamageType,
            spells::{SpellComponents, SpellSchool, SpellTradition, SpellType},
            traits::{Rarity, Traits},
        },
        tests::{assert_eq_ignore_linebreaks, read_test_file, DESCRIPTIONS},
    };
    use std::collections::BTreeMap;

    #[test]
    fn test_render_budget_dahak() {
        let dargon: Npc =
            serde_json::from_str(&read_test_file("pathfinder-bestiary.db/ancient-red-dragon.json")).expect("Deserialization failed");
        let dargon = match dargon {
            Npc::Creature(c) => c,
            _ => panic!("Should have been a creature"),
        };
        assert_eq_ignore_linebreaks(
            &render_creature(&dargon, &DESCRIPTIONS),
            include_str!("../../tests/html/budget_dahak.html"),
        );
    }

    #[test]
    fn attack_render_test() {
        let attacks = [Attack {
            modifier: 10,
            damage: vec![
                CreatureDamage {
                    damage: "2d6".to_string(),
                    damage_type: DamageType::Slashing,
                },
                CreatureDamage {
                    damage: "10d1 + 12".to_string(),
                    damage_type: DamageType::Chaotic,
                },
            ],
            name: "Laz0r".to_string(),
            traits: Traits {
                misc: vec!["chaotic".to_string(), "magical".to_string()],
                rarity: Rarity::Common,
                alignment: None,
                size: None,
            },
        }];
        let mut s = String::new();
        render_attacks(&attacks, &mut s);
        assert_eq!("<b>Laz0r</b> <img alt=\"One Action\" class=\"actionimage\" src=\"/static/actions/OneAction.webp\"> +10 (+5, +0) to hit (chaotic, magical) 2d6 Slashing + 10d1 + 12 Chaotic<br/><hr/>", s);
    }

    #[test]
    fn map_calculation_test() {
        assert_eq!((5, 1, -3), calculate_maps(5, &["chaotic".to_string(), "agile".to_string()]));
        assert_eq!((5, 0, -5), calculate_maps(5, &["chaotic".to_string(), "finesse".to_string()]));
        assert_eq!((0, -4, -8), calculate_maps(0, &["chaotic".to_string(), "agile".to_string()]));
        assert_eq!((69, 64, 59), calculate_maps(69, &[]));
    }

    #[test]
    fn spellcasting_render_test() {
        let mut s = String::new();
        let spellcasting = SpellCasting {
            name: "Arcane Innate Spells".to_string(),
            dc: 42,
            spells: vec![
                Spell {
                    name: "Read Aura".to_string(),
                    area: crate::data::spells::Area::None,
                    basic_save: false,
                    area_string: None,
                    components: SpellComponents {
                        somatic: true,
                        verbal: true,
                        material: false,
                    },
                    cost: String::new(),
                    category: crate::data::spells::SpellCategory::Spell,
                    description: String::new(),
                    duration: String::new(),
                    level: 1,
                    prepared_level: None,
                    range: "30 feet".to_string(),
                    save: None,
                    school: SpellSchool::Divination,
                    secondary_casters: String::new(),
                    secondary_check: String::new(),
                    spell_type: SpellType::Utility,
                    sustained: false,
                    target: "1 object".to_string(),
                    time: "1 Minute".to_string(),
                    primary_check: String::new(),
                    traditions: vec![
                        SpellTradition::Arcane,
                        SpellTradition::Divine,
                        SpellTradition::Occult,
                        SpellTradition::Primal,
                    ],
                    traits: Traits {
                        misc: vec!["Divination".to_string(), "cantrip".to_string(), "detection".to_string()],
                        rarity: Rarity::Common,
                        alignment: None,
                        size: None,
                    },
                    source: String::new(),
                },
                Spell {
                    name: "Wall of Fire".to_string(),
                    area: crate::data::spells::Area::None,
                    basic_save: false,
                    area_string: None,
                    components: SpellComponents {
                        somatic: true,
                        verbal: true,
                        material: true,
                    },
                    cost: String::new(),
                    category: crate::data::spells::SpellCategory::Spell,
                    description: String::new(),
                    duration: String::new(),
                    level: 8,
                    prepared_level: None,
                    range: String::new(),
                    save: None,
                    school: SpellSchool::Evocation,
                    secondary_casters: String::new(),
                    secondary_check: String::new(),
                    spell_type: SpellType::Utility,
                    sustained: false,
                    target: String::new(),
                    time: "3".to_string(),
                    primary_check: String::new(),
                    traditions: vec![SpellTradition::Arcane, SpellTradition::Primal],
                    traits: Traits {
                        misc: vec!["Evocation".to_string(), "fire".to_string()],
                        rarity: Rarity::Common,
                        alignment: None,
                        size: None,
                    },
                    source: String::new(),
                },
            ],
            id: String::new(),
            slots: {
                let mut slots = BTreeMap::new();
                slots.insert(8, 4);
                slots
            },
            casting_type: crate::data::creature::SpellCastingType::Spontaneous,
        };
        render_spells(&spellcasting, &mut s, 16);
        assert_eq_ignore_linebreaks(
            &s,
            "
        <b>Arcane Innate Spells (DC 42)</b><br/>
        <p>
        <b>Cantrips (8th Level):</b> <a href=\"/spell/read_aura\">Read Aura</a><br/>
        <b>8th Level (4 slots):</b> <a href=\"/spell/wall_of_fire\">Wall of Fire</a><br/>
        </p>",
        );
    }
}
