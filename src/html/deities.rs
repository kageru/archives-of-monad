use super::Template;
use crate::HTML_FORMATTING_TAGS;
use crate::{
    data::{deities::Deity, HasName},
    get_data_path,
};
use itertools::Itertools;
use std::{
    borrow::Cow,
    fs,
    io::{self, BufReader},
};

impl Template<()> for Deity {
    fn render(&self, _: ()) -> Cow<'_, str> {
        Cow::Borrowed(&self.content)
    }
}

pub fn render_deities(source: &str, target: &str) -> io::Result<()> {
    fs::create_dir_all(target)?;
    let mut all_deities = fs::read_dir(&format!("{}/packs/data/{}", get_data_path(), source))?
        .filter_map(|f| {
            let filename = f.ok()?.path();
            let f = fs::File::open(&filename).ok()?;
            let reader = BufReader::new(f);
            let deity: Deity = serde_json::from_reader(reader).expect("Deserialization failed");
            Some(deity)
        })
        .collect_vec();
    all_deities.sort_by_key(|s| s.name.clone());
    let mut index = String::with_capacity(10_000);
    index.push_str("<div id=\"gridlist\">");
    for deity in &all_deities {
        let rendered = deity.render(());
        fs::write(format!("{}/{}", target, deity.url_name()), rendered.as_ref())?;
        index.push_str(&format!(
            "<span><a href=\"{}\">{}</a></span>",
            deity.url_name(),
            HTML_FORMATTING_TAGS.replace_all(deity.content.lines().next().unwrap_or_else(|| deity.name()), "")
        ));
    }
    fs::write(format!("{}/index.html", target), &index)
}

#[cfg(test)]
mod tests {
    use crate::tests::read_test_file;

    use super::*;

    #[test]
    fn test_deity_template() {
        let asmodeus: Deity = serde_json::from_str(&read_test_file("deities.db/asmodeus.json")).expect("Deserialization failed");
        let expected = include_str!("../../tests/html/asmodeus.html");
        assert_eq!(asmodeus.render(()).lines().collect::<String>(), expected.lines().collect::<String>());
    }
}
