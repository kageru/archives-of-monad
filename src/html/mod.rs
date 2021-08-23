use crate::{
    data::{
        traits::{Rarity, TraitDescriptions, Traits},
        HasName,
    },
    get_data_path,
};
use convert_case::{Case, Casing};
use serde::de::DeserializeOwned;
use std::{
    borrow::Cow,
    fs,
    io::{self, BufReader},
};

pub(crate) mod actions;
pub(crate) mod archetypes;
pub(crate) mod backgrounds;
pub(crate) mod classes;
pub(crate) mod conditions;
pub(crate) mod deities;
pub(crate) mod feats;
pub(crate) mod spells;

pub(crate) trait Template<AdditionalData>
where
    Self: Sized,
{
    fn render(&self, d: AdditionalData) -> Cow<'_, str>;

    fn render_index(elements: &[Self]) -> String;

    // noop by default
    fn render_subindices(_target: &str, _elements: &[Self]) -> io::Result<()> {
        Ok(())
    }
}

fn read_data<T: DeserializeOwned + Ord>(folder: &str) -> io::Result<Vec<T>> {
    let mut objects = fs::read_dir(&format!("{}/packs/data/{}", get_data_path(), folder))?
        .map(|f| {
            let filename = f?.path();
            let f = fs::File::open(&filename)?;
            let reader = BufReader::new(f);
            let t = serde_json::from_reader(reader).expect("Deserialization failed");
            Ok(t)
        })
        .collect::<io::Result<Vec<T>>>()?;
    objects.sort();
    Ok(objects)
}

pub(crate) fn render<T: Template<Additional> + DeserializeOwned + Ord + HasName, Additional: Copy>(
    folder: &str,
    target: &str,
    additional_data: Additional,
) -> io::Result<Vec<T>> {
    fs::create_dir_all(target)?;
    let elements: Vec<T> = read_data(folder)?;
    fs::write(format!("{}/index.html", target), Template::render_index(&elements))?;
    Template::render_subindices(target, &elements)?;
    for e in &elements {
        fs::write(format!("{}/{}", target, e.url_name()), e.render(additional_data).as_bytes())?;
    }
    Ok(elements)
}

pub fn render_traits(page: &mut String, traits: &Traits) {
    render_traits_in(page, traits, "<div class=\"traits\">", "</div>");
}

pub fn render_traits_inline(page: &mut String, traits: &Traits) {
    render_traits_in(page, traits, "<span class=\"traits-inline\">", "</span>");
}

// No format!() here because there are called often, so the performance might actually matter
fn render_traits_in(page: &mut String, traits: &Traits, open_element: &str, close_element: &str) {
    page.push_str(open_element);
    match traits.rarity {
        Some(Rarity::Common) => (),
        Some(r) => {
            let rarity = r.to_string();
            page.push_str("<span class=\"trait rarity-");
            page.push_str(&rarity.to_lowercase());
            page.push_str("\">");
            page.push_str(&rarity);
            page.push_str("</span>");
        }
        None => (),
    }
    for t in &traits.value {
        page.push_str("<span class=\"trait\">");
        page.push_str(&t.to_case(Case::Pascal));
        page.push_str("</span>");
    }
    page.push_str(close_element);
}

pub fn render_trait_legend(mut page: &mut String, traits: &Traits, trait_descriptions: &TraitDescriptions) {
    page.push_str("<h2>Traits</h2><div class=\"trait-legend\">");
    if let Some(r) = traits.rarity {
        let rarity = r.to_string();
        page.push_str("<b>");
        page.push_str(&rarity);
        page.push_str("</b><p>");
        page.push_str(&trait_descriptions.0[&rarity]);
        page.push_str("</p>");
    };
    page = traits
        .value
        .iter()
        .map(|name| name.to_case(Case::Pascal))
        // The rarity is sometimes redundantly included in the traits. Filter it here.
        .filter(|name| !matches!(traits.rarity.map(|r| r.to_string()), Some(n) if &n == name))
        .filter_map(|name| trait_descriptions.0.get(&name).cloned().map(|s| (name, s)))
        .fold(page, |p, (name, description)| {
            p.push_str("<b>");
            p.push_str(&name);
            p.push_str("</b><p>");
            p.push_str(&description);
            p.push_str("</p>");
            p
        });
    page.push_str("</div>");
}
