use super::Template;
use crate::{
    data::{feats::Feat, traits::TraitDescriptions, HasName},
    get_data_path,
    html::{render_trait_legend, render_traits},
};
use std::{
    borrow::Cow,
    fs,
    io::{self, BufReader},
};

pub fn render_feats(folder: &str, target: &str, trait_descriptions: &TraitDescriptions) -> io::Result<Vec<Feat>> {
    fs::create_dir_all(target)?;
    let mut all_feats = fs::read_dir(&format!("{}/packs/data/{}", get_data_path(), folder))?
        .map(|f| {
            let filename = f?.path();
            let f = fs::File::open(&filename)?;
            let reader = BufReader::new(f);
            let feat = serde_json::from_reader(reader).expect("Deserialization failed");
            Ok(feat)
        })
        .collect::<io::Result<Vec<Feat>>>()?;
    all_feats.sort_by_key(|s| s.name.clone());
    fs::write(format!("{}/index.html", target), render_feat_list(&all_feats))?;
    for feat in &all_feats {
        fs::write(
            format!("{}/{}", target, feat.url_name()),
            feat.render(trait_descriptions).as_bytes(),
        )?;
    }
    Ok(all_feats)
}

fn render_feat_list(feats: &[Feat]) -> String {
    let mut page = String::with_capacity(50_000);
    page.push_str("<div id=\"gridlist\">");
    for feat in feats {
        page.push_str(&format!(
            "<span><a href=\"{}\">{} {}</a></span>",
            feat.url_name(),
            feat.name(),
            feat.action_type.img(&feat.actions)
        ));
    }
    page.push_str("</div>");
    page
}

impl Template<&TraitDescriptions> for Feat {
    fn render(&self, trait_descriptions: &TraitDescriptions) -> Cow<'_, str> {
        let mut page = String::with_capacity(5000);
        page.push_str(&format!(
            "<h1>{} {}<span class=\"type\">Feat {}</span></h1><hr/>",
            &self.name,
            self.action_type.img(&self.actions),
            self.level
        ));
        render_traits(&mut page, &self.traits);
        if !self.prerequisites.is_empty() {
            page.push_str(&self.prerequisites.join(","));
        }
        page.push_str(&self.description);
        page.push_str("<hr/>");
        render_trait_legend(&mut page, &self.traits, trait_descriptions);
        Cow::Owned(page)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{read_test_file, DESCRIPTIONS};

    #[test]
    fn test_feat_template() {
        let feat: Feat = serde_json::from_str(&read_test_file("feats.db/sever-space.json")).expect("Deserialization failed");
        let expected = include_str!("../../tests/html/sever_space.html");
        assert_eq!(
            feat.render(&DESCRIPTIONS).lines().collect::<String>(),
            expected.lines().collect::<String>()
        );
    }
}
