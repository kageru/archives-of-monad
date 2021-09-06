use strum::IntoEnumIterator;

use super::Template;
use crate::{
    data::{
        equipment::{Equipment, ItemType, WeaponType},
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
            page.push_str(&format!("<b>Damage</b> {}", damage));
            if self.splash_damage != 0 {
                page.push_str(&format!(" (plus {} splash damage)", self.splash_damage));
            }
            page.push_str("<br/>");
        }
        if self.weapon_type != WeaponType::NotAWeapon {
            page.push_str(&format!("<b>Type</b> {}<br/>", self.weapon_type.as_ref()));
        }
        if self.range != 0 {
            page.push_str("<b>Range</b> ");
            page.push_str(&self.range.to_string());
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
        for category in ItemType::iter() {
            write_full_page(
                &format!("{}/{}_index", target, category.as_ref()),
                &format!("{} List", category.as_ref()),
                &render_filtered_index(category.as_ref(), elements, |e| e.item_type == category),
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
    page.push_str("<thead><tr><td>Name</td><td class=\"traitcolumn\">Traits</td><td>Value</td><td>Type</td><td>Level</td></tr></thead>");
    for (item, _) in elements.iter().filter(|(i, _)| filter(i)) {
        page.push_str(&format!(
            "<tr><td><a href=\"{}\">{}</a></td><td class=\"traitcolumn\">",
            item.url_name(),
            item.name,
        ));
        render_traits_inline(&mut page, &item.traits);
        page.push_str(&format!(
            "</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            item.format_price().unwrap_or(Cow::Borrowed("")),
            item.category(),
            item.level,
        ));
    }
    page.push_str("</table>");
    page
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{read_test_file, DESCRIPTIONS};

    #[test]
    fn test_item_template() {
        let blackaxe: Equipment = serde_json::from_str(&read_test_file("equipment.db/blackaxe.json")).expect("Deserialization failed");
        let expected: String = include_str!("../../tests/html/blackaxe.html").lines().collect();
        assert_eq!(expected, blackaxe.render(&DESCRIPTIONS).lines().collect::<String>());
    }

    #[test]
    fn test_item_with_splash() {
        let bomb: Equipment =
            serde_json::from_str(&read_test_file("equipment.db/necrotic-bomb-major.json")).expect("Deserialization failed");
        let expected: String = include_str!("../../tests/html/necrotic_bomb.html").lines().collect();
        assert_eq!(expected, bomb.render(&DESCRIPTIONS).lines().collect::<String>());
    }

    #[test]
    fn test_item_hp() {
        let shield: Equipment =
            serde_json::from_str(&read_test_file("equipment.db/shield-of-the-unified-legion.json")).expect("Deserialization failed");
        let expected: String = include_str!("../../tests/html/shield_of_unified_legion.html").lines().collect();
        assert_eq!(expected, shield.render(&DESCRIPTIONS).lines().collect::<String>());
    }
}
