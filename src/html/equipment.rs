use super::Template;
use crate::{data::{equipment::Equipment, HasName}, html::render_traits};
use itertools::Itertools;
use std::borrow::Cow;

/*
    pub name: String,
    pub damage: Option<EquipmentDamage>,
    pub description: String,
    pub group: WeaponGroup,
    pub hardness: i32,
    pub max_hp: i32,
    pub level: i32,
    pub price: String, // e.g. "2 sp"
    pub range: i32,
    pub splash_damage: i32,
    pub traits: Traits,
    pub usage: Option<ItemUsage>,
    pub weapon_type: WeaponType,
    pub weight: Weight,
    pub item_type: ItemType,
*/
impl Template<()> for Equipment {
    fn render(&self, _: ()) -> Cow<'_, str> {
        let mut page = String::with_capacity(1000);
        page.push_str(&format!(
            "<h1>{}<span class=\"type\">{} {}</span></h1><hr/>",
            &self.name, &self.item_type, &self.level
        ));
        render_traits(&mut page, &self.traits);
        if let Some(damage) = &self.damage {
            page.push_str(&format!("<b>Damage</b> {}<br/>", damage))
        }
        page.push_str("<b>Weight</b> ");
        page.push_str(&self.weight.to_string());
        page.push_str("<br/>");
        page.push_str("<hr/>");
        page.push_str(&self.description);
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
    use crate::tests::read_test_file;

    #[test]
    fn test_item_template() {
        let blackaxe: Equipment = serde_json::from_str(&read_test_file("equipment.db/blackaxe.json")).expect("Deserialization failed");
        let expected: String = include_str!("../../tests/html/blackaxe.html").lines().collect();
        assert_eq!(expected, blackaxe.render(()).lines().collect::<String>());
    }
}
