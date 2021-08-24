use super::Template;
use crate::{
    data::{equipment::Equipment, traits::TraitDescriptions, HasName},
    html::{render_trait_legend, render_traits},
};
use itertools::Itertools;
use std::borrow::Cow;

/*
 * Missing:
 * pub group: WeaponGroup,
 * pub usage: Option<ItemUsage>,
 * pub weapon_type: WeaponType,
*/
impl Template<&TraitDescriptions> for Equipment {
    fn render(&self, trait_descriptions: &TraitDescriptions) -> Cow<'_, str> {
        let mut page = String::with_capacity(1000);
        page.push_str(&format!(
            "<h1>{}<span class=\"type\">{} {}</span></h1><hr/>",
            &self.name, &self.item_type, &self.level
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
        if self.range != 0 {
            page.push_str("<b>Range</b> ");
            page.push_str(&self.range.to_string());
            page.push_str("<br/>");
        }
        if !self.price.is_empty() && !self.price.starts_with("0") {
            page.push_str("<b>Price</b> ");
            page.push_str(&self.price);
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

    fn render_index(elements: &[Self]) -> String {
        let mut page = String::with_capacity(20_000);
        page.push_str("<h1>Equipment</h1><hr><br/><div id=\"list\">");
        for (level, items) in &elements.iter().group_by(|i| i.level) {
            page.push_str(&format!("<h2>Level {}</h2><hr>", level));
            for item in items {
                page.push_str(&format!("<p><a href=\"{}\">{}</a></p>", item.url_name(), item.name));
            }
        }
        page.push_str("</div>");
        page
    }
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
