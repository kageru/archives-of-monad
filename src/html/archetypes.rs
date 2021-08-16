use super::Template;
use crate::data::HasName;
use crate::{data::archetypes::Archetype, get_data_path};
use std::{borrow::Cow, io::BufReader};
use std::{fs, io};

impl Template<()> for Archetype {
    fn render(&self, _: ()) -> Cow<'_, str> {
        Cow::Owned(format!(
            "<h1>{}<span class=\"type\">Archetype</span></h1><hr/>{}",
            &self.name, &self.content
        ))
    }
}

pub fn render_archetypes(folder: &str, target: &str) -> io::Result<()> {
    fs::create_dir_all(target)?;
    let mut all_archetypes = fs::read_dir(&format!("{}/packs/data/{}", get_data_path(), folder))?
        .map(|f| {
            let filename = f?.path();
            let f = fs::File::open(&filename)?;
            let reader = BufReader::new(f);
            let archetype = serde_json::from_reader(reader).expect("Deserialization failed");
            Ok(archetype)
        })
        .collect::<io::Result<Vec<Archetype>>>()?;
    all_archetypes.sort_by_key(|s| s.name.clone());
    for archetype in &all_archetypes {
        fs::write(format!("{}/{}", target, archetype.url_name()), archetype.render(()).as_bytes())?;
    }
    fs::write(format!("{}/index.html", target), render_archetype_list(&all_archetypes))
}

// TODO: proper archetype list
fn render_archetype_list(all_archetypes: &[Archetype]) -> String {
    let mut page = String::with_capacity(10_000);
    page.push_str("<div id=\"gridlist\">");
    for archetype in all_archetypes {
        page.push_str(&format!(
            "<span><a href=\"{}\">{}</a></span>",
            archetype.url_name(),
            archetype.name(),
        ));
    }
    page.push_str("</div>");
    page
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
}
