use super::Template;
use crate::{
    data::{classes::Class, traits::TraitDescriptions, HasName},
    get_data_path,
};
use std::{
    borrow::Cow,
    fs,
    io::{self, BufReader},
};

impl Template<&TraitDescriptions> for Class {
    fn render(&self, _: &TraitDescriptions) -> Cow<'_, str> {
        Cow::Borrowed(&self.description)
    }
}

pub fn render_classes(folder: &str, target: &str, trait_descriptions: &TraitDescriptions) -> io::Result<Vec<Class>> {
    fs::create_dir_all(target)?;
    let mut classes = fs::read_dir(&format!("{}/packs/data/{}", get_data_path(), folder))?
        .map(|f| {
            let filename = f?.path();
            println!("Reading {}", filename.to_str().unwrap());
            let f = fs::File::open(&filename)?;
            let reader = BufReader::new(f);
            let class = serde_json::from_reader(reader).expect("Deserialization failed");
            Ok(class)
        })
        .collect::<io::Result<Vec<Class>>>()?;
    classes.sort_by_key(|s| s.name.clone());
    for class in &classes {
        fs::write(
            format!("{}/{}", target, class.url_name()),
            class.render(trait_descriptions).as_bytes(),
        )?;
    }
    fs::write(format!("{}/index.html", target), render_class_list(&classes))?;
    Ok(classes)
}

fn render_class_list(classes: &[Class]) -> String {
    let mut page = String::with_capacity(1000);
    page.push_str("<h1>Classes</h1><hr/><div id=\"list\">");
    for class in classes {
        page.push_str(&format!("<h2><a href=\"{}\">{}</a></h2><br/>", class.url_name(), class.name));
    }
    page.push_str("</div>");
    page
}
