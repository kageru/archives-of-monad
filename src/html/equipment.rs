use itertools::Itertools;
use strum::IntoEnumIterator;

use super::Template;
use crate::{
    data::{
        damage::EquipmentDamageWithSplash,
        ensure_trailing_unit,
        equipment::{Equipment, ItemType, ProficiencyGroup},
        traits::TraitDescriptions,
        HasName,
    },
    html::{render_trait_legend, render_traits, render_traits_inline, write_full_page, Page},
};
use std::borrow::Cow;

/*
 * Missing:
 * pub group: WeaponGroup,
 * pub usage: Option<ItemUsage>,
*/
impl Template<&TraitDescriptions> for Equipment {
    fn render(&self, trait_descriptions: &TraitDescriptions) -> Cow<'_, str> {
        let mut page = String::with_capacity(1000);
        page.push_str(&format!(
            "<h1><a href=\"/item/{}\">{}</a><span class=\"type\">{} {}</span></h1><hr/>",
            self.url_name(),
            &self.name,
            &self.category(),
            &self.level
        ));
        render_traits(&mut page, &self.traits);
        if !self.source.is_empty() {
            page.push_str(&format!("<b>Source</b> {}<br/>", self.source));
        }
        if self.max_hp != 0 {
            page.push_str("<b>Hit points</b> ");
            page.push_str(&self.max_hp.to_string());
            page.push_str(" (");
            if self.hardness != 0 {
                page.push_str("Hardness ");
                page.push_str(&self.hardness.to_string());
                page.push_str(", ");
            }
            page.push_str("BT ");
            page.push_str(&(self.max_hp / 2).to_string());
            page.push_str(")<br/>");
        }
        if let Some(damage) = &self.damage {
            page.push_str(&EquipmentDamageWithSplash(damage, self.splash_damage).to_string());
            page.push_str("<br/>");
        }
        if self.category != ProficiencyGroup::NotAWeapon {
            page.push_str(&format!("<b>Type</b> {}<br/>", self.category.as_ref()));
        }
        if self.range != 0 {
            page.push_str("<b>Range</b> ");
            page.push_str(&ensure_trailing_unit(&self.range.to_string()));
            page.push_str("<br/>");
        }
        if let Some(price) = self.format_price() {
            page.push_str("<b>Price</b> ");
            page.push_str(&price);
            page.push_str("<br/>");
        }
        page.push_str("<b>Weight</b> ");
        page.push_str(&self.weight.to_string());
        page.push_str("<br/>");
        page.push_str("<hr/>");
        page.push_str(&self.description);
        render_trait_legend(&mut page, &self.traits, trait_descriptions);
        Cow::Owned(page)
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.item_type.into())
    }

    fn render_index(elements: &[(Self, Page)]) -> String {
        render_filtered_index("Equipment", elements, |_| true)
    }

    fn render_subindices(target: &str, elements: &[(Self, Page)]) -> std::io::Result<()> {
        for category in ItemType::iter().filter(|&t| t != ItemType::Weapon) {
            write_full_page(
                &format!("{}/{}_index", target, category.as_ref()),
                &format!("{} List", category.as_ref()),
                &render_filtered_index(category.as_ref(), elements, |e| e.item_type == category),
            )?;
        }
        write_full_page(
            &format!("{}/{}_index", target, ItemType::Weapon.as_ref()),
            &format!("{} List", ItemType::Weapon.as_ref()),
            &render_weapon_index(elements),
        )?;
        for t in elements.iter().flat_map(|(i, _)| &i.traits.misc).unique() {
            let title = &format!("{} Items", t);
            write_full_page(
                &format!("{}/trait_{}", target, t.to_lowercase()),
                title,
                &render_filtered_index(title, elements, |e| e.traits.misc.contains(t)),
            )?;
        }
        Ok(())
    }
}

fn add_item_header(page: &mut String) {
    page.push_str(r#"<div class="header">"#);
    page.push_str(r#"<span><a href="index.html"><div>All</div></a></span>"#);
    for item_type in ItemType::iter() {
        page.push_str(&format!(
            r#"<span><a href="/item/{}_index"><div>{}</div></a></span>"#,
            item_type.as_ref(),
            item_type.as_ref()
        ));
    }
    page.push_str("</div>");
}

fn render_filtered_index<F: FnMut(&Equipment) -> bool>(title: &str, elements: &[(Equipment, Page)], mut filter: F) -> String {
    let mut page = String::with_capacity(250_000);
    add_item_header(&mut page);
    page.push_str("<h1>");
    page.push_str(title);
    page.push_str("</h1><hr><br/><br/>");
    page.push_str("<table class=\"overview\">");
    page.push_str("<thead><tr><td>Name</td><td class=\"traitcolumn\">Traits</td><td>Value</td><td>Type</td><td>Source</td><td>Level</td></tr></thead>");
    for (item, _) in elements.iter().filter(|(i, _)| filter(i)) {
        page.push_str(&format!(
            "<tr><td><a href=\"{}\">{}</a></td><td class=\"traitcolumn\">",
            item.url_name(),
            item.name,
        ));
        render_traits_inline(&mut page, &item.traits);
        page.push_str(&format!(
            "</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            item.format_price().unwrap_or(Cow::Borrowed("")),
            item.category(),
            item.source,
            item.level,
        ));
    }
    page.push_str("</table>");
    page
}

fn render_weapon_index(elements: &[(Equipment, Page)]) -> String {
    let mut page = String::with_capacity(100_000);
    add_item_header(&mut page);
    page.push_str(
        "<h1>Weapons</h1><hr><br/><br/>
        <table class=\"overview\">
        <thead><tr><td>Name</td><td class=\"traitcolumn\">Traits</td><td>Weapon Group</td><td>Damage</td><td>Value</td><td>Type</td><td>Range</td><td>Level</td></tr></thead>",
    );
    for (item, _) in elements
        .iter()
        .filter(|(i, _)| i.item_type == ItemType::Weapon)
        .sorted_by_key(|&(i, _)| match i.category {
            ProficiencyGroup::Unarmed => 0,
            ProficiencyGroup::Simple => 1,
            ProficiencyGroup::Martial => 2,
            ProficiencyGroup::Advanced => 3,
            ProficiencyGroup::NotAWeapon => unreachable!(),
        })
    {
        page.push_str(&format!(
            "<tr><td><a href=\"{}\">{}</a></td><td class=\"traitcolumn\">",
            item.url_name(),
            item.name,
        ));
        render_traits_inline(&mut page, &item.traits);
        page.push_str(&format!(
            "</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            item.group.as_ref(),
            item.damage.clone().map(|d| d.to_string()).unwrap_or_else(String::new),
            item.format_price().unwrap_or(Cow::Borrowed("")),
            item.category.as_ref(),
            if item.range == 0 {
                "Melee".to_string()
            } else {
                format!("{} feet", item.range)
            },
            item.level,
        ));
    }
    page.push_str("</table>");
    page
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_eq_ignore_linebreaks, read_test_file, DESCRIPTIONS};

    #[test]
    fn test_item_template() {
        let blackaxe: Equipment = serde_json::from_str(&read_test_file("equipment.db/blackaxe.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&blackaxe.render(&DESCRIPTIONS), include_str!("../../tests/html/blackaxe.html"));
    }

    #[test]
    fn test_item_with_splash() {
        let bomb: Equipment =
            serde_json::from_str(&read_test_file("equipment.db/necrotic-bomb-major.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&bomb.render(&DESCRIPTIONS), include_str!("../../tests/html/necrotic_bomb.html"));
    }

    #[test]
    fn test_item_hp() {
        let shield: Equipment =
            serde_json::from_str(&read_test_file("equipment.db/shield-of-the-unified-legion.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(
            &shield.render(&DESCRIPTIONS),
            include_str!("../../tests/html/shield_of_unified_legion.html"),
        );
    }
}
