use crate::{
    data::{
        traits::{Rarity, TraitDescriptions, Traits},
        HasName,
    },
    get_data_path, INDEX_REGEX,
};
use convert_case::{Case, Casing};
use itertools::Itertools;
use meilisearch_sdk::document::Document;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    borrow::Cow,
    fs,
    io::{self, BufReader},
};

pub(crate) mod actions;
pub(crate) mod ancestries;
pub(crate) mod archetypes;
pub(crate) mod backgrounds;
pub(crate) mod classes;
pub(crate) mod classfeatures;
pub(crate) mod conditions;
pub(crate) mod deities;
pub(crate) mod equipment;
pub(crate) mod feats;
pub(crate) mod spells;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct Page {
    name: String,
    content: String,
    category: String,
    id: String,
}

impl HasName for Page {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Document for Page {
    type UIDType = String;
    fn get_uid(&self) -> &Self::UIDType {
        &self.id
    }
}

pub(crate) trait Template<AdditionalData>
where
    Self: Sized + Ord + HasName + DeserializeOwned,
{
    fn render(&self, d: AdditionalData) -> Cow<'_, str>;

    fn category(&self) -> Cow<'_, str>;

    fn render_index(elements: &[(Self, Page)]) -> String;

    // noop by default
    fn render_subindices(_target: &str, _elements: &[(Self, Page)]) -> io::Result<()> {
        Ok(())
    }
}

fn read_data<T: DeserializeOwned + Ord>(folder: &str) -> io::Result<Vec<T>> {
    let mut objects = fs::read_dir(&format!("{}/packs/data/{}", get_data_path(), folder))?
        .map(|f| {
            let filename = f?.path();
            let f = fs::File::open(&filename)?;
            let reader = BufReader::new(f);
            // println!("Reading {:?}", filename);
            let t = serde_json::from_reader(reader).expect("Deserialization failed");
            Ok(t)
        })
        .collect::<io::Result<Vec<T>>>()?;
    objects.sort();
    Ok(objects)
}

pub(crate) fn render<T: Template<Additional>, Additional: Copy>(
    folder: &str,
    target: &str,
    additional_data: Additional,
) -> io::Result<Vec<(T, Page)>> {
    fs::create_dir_all(target)?;
    let elements: Vec<T> = read_data(folder)?;
    let pages = elements
        .into_iter()
        .filter(|e| !e.name().starts_with("[Empty"))
        .map(|e| attach_page(e, additional_data))
        .collect_vec();
    fs::write(format!("{}/index.html", target), Template::render_index(&pages))?;
    Template::render_subindices(target, &pages)?;
    for (_, page) in &pages {
        fs::write(format!("{}/{}", target, page.url_name()), page.content.as_bytes())?;
    }
    Ok(pages)
}

pub(crate) fn attach_page<A, T: Template<A>>(e: T, additional_data: A) -> (T, Page) {
    let page = Page {
        name: e.name().to_owned(),
        category: e.category().to_string(),
        id: format!("{}-{}", e.category(), INDEX_REGEX.replace_all(e.name(), "")),
        content: e.render(additional_data).to_string(),
    };
    (e, page)
}

pub fn inline_rarity_if_not_common(rarity: &Option<Rarity>) -> String {
    let mut s = String::with_capacity(100);
    s.push_str("<span class=\"traits-inline\">");
    rarity_if_not_common(&mut s, rarity);
    s.push_str("</span>");
    s
}

pub fn render_traits(page: &mut String, traits: &Traits) {
    render_traits_in(page, traits, "<div class=\"traits\">", "</div>");
}

pub fn render_traits_inline(page: &mut String, traits: &Traits) {
    render_traits_in(page, traits, "<span class=\"traits-inline\">", "</span>");
}

fn rarity_if_not_common(page: &mut String, rarity: &Option<Rarity>) {
    match rarity {
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
}

// No format!() here because there are called often, so the performance might actually matter
fn render_traits_in(page: &mut String, traits: &Traits, open_element: &str, close_element: &str) {
    if (traits.rarity.is_none() || traits.rarity == Some(Rarity::Common)) && traits.value.is_empty() {
        return;
    }
    page.push_str(open_element);
    rarity_if_not_common(page, &traits.rarity);
    let rarity_string = &traits.rarity.map(|r| r.as_str().to_lowercase());
    for t in traits
        .value
        .iter()
        // good candidate for Option::contains if/when that gets stabilized
        .filter(|t| if let Some(r) = rarity_string { &r != t } else { true })
    {
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
