use super::Template;
use crate::data::archetypes::Archetype;
use crate::data::HasName;
use crate::html::Page;
use std::borrow::Cow;

impl Template<()> for Archetype {
    fn render(&self, _: ()) -> Cow<'_, str> {
        Cow::Owned(format!(
            "<h1><a href=\"/archetype/{}\">{}</a><span class=\"type\">Archetype</span></h1><hr/>{}",
            &self.url_name(),
            &self.name,
            &self.content,
        ))
    }

    // TODO: proper archetype list
    fn render_index(elements: &[(Self, Page)]) -> String {
        let mut page = String::with_capacity(10_000);
        page.push_str("<div id=\"gridlist\">");
        for (archetype, _) in elements {
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
    use crate::{
        html::attach_page,
        tests::{assert_eq_ignore_linebreaks, read_test_file},
    };
    use itertools::Itertools;

    #[test]
    fn test_archetype_template() {
        let assassin: Archetype = serde_json::from_str(&read_test_file("archetypes.db/assassin.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&assassin.render(()), include_str!("../../tests/html/assassin.html"));
    }

    #[test]
    fn test_archetype_index() {
        let assassin: Archetype = serde_json::from_str(&read_test_file("archetypes.db/assassin.json")).expect("Deserialization failed");
        let juggler: Archetype = serde_json::from_str(&read_test_file("archetypes.db/juggler.json")).expect("Deserialization failed");
        let archetypes = vec![assassin, juggler].into_iter().map(|a| attach_page(a, ())).collect_vec();
        assert_eq_ignore_linebreaks(
            &Template::render_index(&archetypes),
            include_str!("../../tests/html/archetype_index.html"),
        );
    }
}
