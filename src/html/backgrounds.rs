use super::{render_traits, Template};
use crate::data::backgrounds::Background;
use crate::data::HasName;
use crate::get_data_path;
use std::borrow::Cow;
use std::io::BufReader;
use std::{fs, io};

impl Template<&str> for Background {
    fn render(&self, condensed: &str) -> Cow<'_, str> {
        let mut page = String::with_capacity(1000);
        page.push_str("<h1>");
        page.push_str(&self.name);
        page.push_str("<span class=\"type\">Background</span></h1><hr/>");
        render_traits(&mut page, &self.traits);
        page.push_str(&self.description);
        page.push_str("<b>Condensed:</b><br/>");
        page.push_str(condensed);
        Cow::Owned(page)
    }
}
pub fn render_backgrounds(folder: &str, target: &str) -> io::Result<()> {
    let mut backgrounds = fs::read_dir(&format!("{}/packs/data/{}", get_data_path(), folder))?
        .map(|f| {
            let filename = f?.path();
            let f = fs::File::open(&filename)?;
            let reader = BufReader::new(f);
            let bg: Background = serde_json::from_reader(reader).expect("Deserialization failed");
            Ok(bg)
        })
        .collect::<io::Result<Vec<Background>>>()?;
    backgrounds.sort_by_key(|s| s.name.clone());
    let mut index = String::with_capacity(10_000);
    index.push_str("<h1>Backgrounds</h1><hr/>");
    index.push_str("<div id=\"list\">");
    for bg in &backgrounds {
        let condensed = bg.condensed();
        fs::write(format!("{}/{}", target, bg.url_name()), bg.render(&condensed).as_bytes())?;
        index.push_str("<p>");
        index.push_str(&bg.name);
        index.push_str(" (");
        render_traits(&mut index, &bg.traits);
        index.push_str(&condensed);
        index.push_str(")</p>");
    }
    index.push_str("</div>");
    fs::write("{}/index.html", index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::read_test_file;

    #[test]
    fn test_background_template() {
        let field_medic: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/field-medic.json")).expect("Deserialization of background failed");
        let expected = include_str!("../../tests/html/field_medic.html");
        assert_eq!(
            field_medic.render(&field_medic.condensed()).lines().collect::<String>(),
            expected.lines().collect::<String>(),
        );
    }

    #[test]
    fn test_background_template_haunted() {
        let haunted: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/haunted.json")).expect("Deserialization of background failed");
        let expected = include_str!("../../tests/html/haunted.html");
        assert_eq!(
            haunted.render(&haunted.condensed()).lines().collect::<String>(),
            expected.lines().collect::<String>()
        );
    }
}
