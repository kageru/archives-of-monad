use super::Template;
use crate::data::archetypes::Archetype;
use crate::data::HasName;
use std::borrow::Cow;

impl Template<()> for Archetype {
    fn render(&self, _: ()) -> Cow<'_, str> {
        Cow::Owned(format!(
            "<h1>{}<span class=\"type\">Archetype</span></h1><hr/>{}",
            &self.name, &self.content
        ))
    }

    // TODO: proper archetype list
    fn render_index(elements: &[Self]) -> String {
        let mut page = String::with_capacity(10_000);
        page.push_str("<div id=\"gridlist\">");
        for archetype in elements {
            page.push_str(&format!(
                "<span><a href=\"{}\">{}</a></span>",
                archetype.url_name(),
                archetype.name(),
            ));
        }
        page.push_str("</div>");
        page
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed("Archetype")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::read_test_file;

    #[test]
    fn test_archetype_template() {
        let assassin: Archetype = serde_json::from_str(&read_test_file("archetypes.db/assassin.json")).expect("Deserialization failed");
        let expected = include_str!("../../tests/html/assassin.html");
        assert_eq!(
            assassin.render(()).lines().collect::<String>(),
            expected.lines().collect::<String>()
        );
    }

    #[test]
    fn test_archetype_index() {
        let assassin: Archetype = serde_json::from_str(&read_test_file("archetypes.db/assassin.json")).expect("Deserialization failed");
        let juggler: Archetype = serde_json::from_str(&read_test_file("archetypes.db/juggler.json")).expect("Deserialization failed");
        let expected: String = include_str!("../../tests/html/archetype_index.html").lines().collect();
        assert_eq!(Template::render_index(&[assassin, juggler]), expected);
    }
}
