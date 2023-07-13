use crate::data::ancestries::TraitNew;
use crate::{
    data::{
        traits::{clean_trait_name, Rarity, Traits, Translations},
        HasName,
    },
    get_data_path, get_scraped_data_path, URL_REPLACEMENTS,
};
use convert_case::{Case, Casing};
use itertools::Itertools;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    borrow::Cow,
    fmt, fs,
    io::{self, BufReader, BufWriter, Write},
};

pub(crate) mod actions;
pub(crate) mod ancestries;
pub(crate) mod ancestryfeatures;
pub(crate) mod backgrounds;
pub(crate) mod classes;
pub(crate) mod classfeatures;
pub(crate) mod conditions;
pub(crate) mod creatures;
pub(crate) mod deities;
pub(crate) mod equipment;
pub(crate) mod feats;
pub(crate) mod heritages;
pub(crate) mod spells;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub(crate) struct HtmlPage {
    pub name: String,
    pub content: String,
    pub category: String,
    pub id: String,
}

impl HasName for HtmlPage {
    fn name(&self) -> &str {
        &self.name
    }
}

pub(crate) trait Template<AdditionalData>
where
    Self: Sized + Ord + HasName + DeserializeOwned,
{
    fn render(&self, d: AdditionalData) -> String;

    fn category(&self) -> String;

    fn render_index(elements: &[(Self, HtmlPage)]) -> String;

    // noop by default
    fn render_subindices(_target: &str, _elements: &[(Self, HtmlPage)]) -> io::Result<()> {
        Ok(())
    }

    fn category_url_safe(&self) -> String {
        URL_REPLACEMENTS.replace_all(&self.category(), "").to_string()
    }

    fn header(&self) -> Option<Cow<'_, str>> {
        None
    }
}

fn read_data<T: DeserializeOwned + Ord, P: fmt::Display>(folder: P) -> io::Result<Vec<T>> {
    fs::read_dir(format!("{}/packs/{}", get_data_path(), folder))?
        .map(|f| {
            let filename = f?.path();
            let f = fs::File::open(&filename)?;
            let reader = BufReader::new(f);
            #[cfg(debug_assertions)]
            println!("Reading {:?}", filename);
            let t = serde_json::from_reader(reader)?;
            Ok(t)
        })
        .collect()
}

fn read_scraped_data<T: DeserializeOwned + Ord, P: fmt::Display>(file: P) -> io::Result<Vec<T>> {
    let filename = format!("{}/{}.json", get_scraped_data_path(), file);
    let file = fs::File::open(&filename)?;
    let reader = BufReader::new(file);
    #[cfg(debug_assertions)]
    println!("Reading {:?}", filename);
    let t = serde_json::from_reader(reader)?;
    Ok(t)
}

fn title_from_target_folder(target: &str) -> String {
    target
        .strip_prefix("output/")
        .unwrap_or(target)
        .from_case(Case::Lower)
        .to_case(Case::Title)
}

pub(crate) fn render<T: Template<Additional>, Additional: Copy, P: fmt::Display>(
    folders: &[P],
    target: &str,
    additional_data: Additional,
) -> io::Result<Vec<(T, HtmlPage)>> {
    fs::create_dir_all(target)?;
    let mut elements = folders.iter().map(read_data).flatten_ok().collect::<io::Result<Vec<T>>>()?;
    elements.sort();
    let pages = elements
        .into_iter()
        .filter(|e| !e.name().starts_with("[Empty"))
        .map(|e| attach_html(e, additional_data))
        .filter(|(_, p)| !p.content.is_empty())
        .collect_vec();
    Template::render_subindices(target, &pages)?;
    write_full_html_document(
        &format!("{}/index.html", target),
        &format!("{} List", title_from_target_folder(target)),
        &Template::render_index(&pages),
    )?;
    for (e, page) in &pages {
        if let Some(header) = e.header() {
            write_full_html_document_with_header(&format!("{}/{}", target, page.url_name()), e.name(), &page.content, &header)?;
        } else {
            write_full_html_document(&format!("{}/{}", target, page.url_name()), e.name(), &page.content)?;
        }
    }
    Ok(pages)
}

pub(crate) fn render_scraped<T: Template<Additional>, Additional: Copy, P: fmt::Display>(
    folders: &[P],
    target: &str,
    additional_data: Additional,
) -> io::Result<Vec<(T, HtmlPage)>> {
    fs::create_dir_all(target)?;
    let mut elements = folders.iter().map(read_scraped_data).flatten_ok().collect::<io::Result<Vec<T>>>()?;
    elements.sort();
    let pages = elements
        .into_iter()
        .filter(|e| !e.name().starts_with("[Empty"))
        .map(|e| attach_html(e, additional_data))
        .filter(|(_, p)| !p.content.is_empty())
        .collect_vec();
    Template::render_subindices(target, &pages)?;
    write_full_html_document(
        &format!("{}/index.html", target),
        &format!("{} List", title_from_target_folder(target)),
        &Template::render_index(&pages),
    )?;
    for (e, page) in &pages {
        if let Some(header) = e.header() {
            write_full_html_document_with_header(&format!("{}/{}", target, page.url_name()), e.name(), &page.content, &header)?;
        } else {
            write_full_html_document(&format!("{}/{}", target, page.url_name()), e.name(), &page.content)?;
        }
    }
    Ok(pages)
}

pub(crate) fn attach_html<A, T: Template<A>>(e: T, additional_data: A) -> (T, HtmlPage) {
    let page = HtmlPage {
        name: e.name().to_owned(),
        category: e.category(),
        id: format!("{}-{}", e.category_url_safe(), URL_REPLACEMENTS.replace_all(e.name(), "")),
        content: e.render(additional_data),
    };
    (e, page)
}

pub fn inline_rarity_if_not_common(rarity: &Rarity) -> String {
    if rarity == &Rarity::Common {
        return String::new();
    }
    let mut s = String::with_capacity(100);
    s.push_str("<span class=\"traits-inline\">");
    rarity_if_not_common(&mut s, rarity);
    s.push_str("</span>");
    s
}

pub fn render_traits(page: &mut String, traits: &Traits) {
    render_traits_in(page, traits, "<div class=\"traits\">", "</div>");
}

pub fn render_traits_new(page: &mut String, traits: &[TraitNew]) {
    render_traits_in_new(page, traits, "<div class=\"traits\">", "</div>");
}

pub fn render_traits_inline(page: &mut String, traits: &Traits) {
    render_traits_in(page, traits, "<span class=\"traits-inline\">", "</span>");
}

fn rarity_if_not_common(page: &mut String, rarity: &Rarity) {
    if rarity != &Rarity::Common {
        page.push_str("<span class=\"trait rarity-");
        page.push_str(&rarity.as_ref().to_lowercase());
        page.push_str("\">");
        page.push_str(rarity.as_ref());
        page.push_str("</span>");
        page.push(ZERO_WIDTH_BREAKING_SPACE);
    }
}

// This is a zero-width space that allows browser to linewrap between these spans if necessary
const ZERO_WIDTH_BREAKING_SPACE: char = '\u{200B}';

// No format!() here because there are called often, so the performance might actually matter
fn render_traits_in(page: &mut String, traits: &Traits, open_element: &str, close_element: &str) {
    if traits.rarity == Rarity::Common && traits.misc.is_empty() && traits.alignment.is_none() && traits.size.is_none() {
        return;
    }
    page.push_str(open_element);
    rarity_if_not_common(page, &traits.rarity);
    if let Some(alignment) = traits.alignment {
        page.push_str("<span class=\"trait trait-alignment\">");
        page.push_str(alignment.as_ref());
        page.push_str("</span>");
        page.push(ZERO_WIDTH_BREAKING_SPACE);
    }
    if let Some(size) = traits.size {
        page.push_str("<span class=\"trait trait-size\">");
        page.push_str(size.as_ref());
        page.push_str("</span>");
        page.push(ZERO_WIDTH_BREAKING_SPACE);
    }
    render_misc_traits(traits, page);
    page.push_str(close_element);
}

fn render_traits_in_new(page: &mut String, traits: &[TraitNew], open_element: &str, close_element: &str) {
    page.push_str(open_element);
    render_misc_traits_new(traits, page);
    page.push_str(close_element);
}

fn render_misc_traits(traits: &Traits, page: &mut String) {
    let rarity_string = traits.rarity.as_ref().to_lowercase();
    for t in traits.misc.iter().filter(|t| t != &&rarity_string) {
        page.push_str("<a href=\"trait_");
        page.push_str(&t.to_lowercase());
        page.push_str("\"><span class=\"trait\">");
        page.push_str(&t.to_case(Case::Pascal));
        page.push_str("</span></a>");
        page.push(ZERO_WIDTH_BREAKING_SPACE);
    }
}

fn render_misc_traits_new(traits: &[TraitNew], page: &mut String) {
    for t in traits.iter() {
        page.push_str("<a href=\"trait_");
        page.push_str(&t.name.to_lowercase());
        page.push_str("\"><span class=\"trait\">");
        page.push_str(&t.name.to_case(Case::Pascal));
        page.push_str("</span></a>");
        page.push(ZERO_WIDTH_BREAKING_SPACE);
    }
}

pub fn render_trait_legend(mut page: &mut String, traits: &Traits, trait_descriptions: &Translations) {
    page.push_str("<h2>Traits</h2><div class=\"trait-legend\">");
    let rarity = traits.rarity.as_ref();
    page.push_str("<b>");
    page.push_str(rarity);
    page.push_str("</b><p>");
    page.push_str(&trait_descriptions.traits[rarity]);
    page.push_str("</p>");
    page = traits
        .misc
        .iter()
        .map(|t| clean_trait_name(t))
        .map(|name| name.to_case(Case::Pascal))
        // The rarity is sometimes redundantly included in the traits. Filter it here.
        .filter(|name| traits.rarity.as_ref() != name)
        .filter_map(|name| trait_descriptions.traits.get(&name).map(|s| (name, s)))
        .fold(page, |p, (name, description)| {
            p.push_str("<b>");
            p.push_str(&name);
            p.push_str("</b><p>");
            p.push_str(description);
            p.push_str("</p>");
            p
        });
    page.push_str("</div>");
}

pub fn write_full_html_document_with_header(path: &str, title: &str, content: &str, header: &str) -> io::Result<()> {
    let index_file = fs::File::create(path)?;
    let mut writer = BufWriter::new(index_file);
    write_head(&mut writer, title)?;
    writer.write_all(header.as_bytes())?;
    writer.write_all(content.as_bytes())?;
    writer.write_all(AFTER_BODY.as_bytes())?;
    Ok(())
}

pub fn write_full_html_document(path: &str, title: &str, content: &str) -> io::Result<()> {
    let index_file = fs::File::create(path)?;
    let mut writer = BufWriter::new(index_file);
    write_head(&mut writer, title)?;
    writer.write_all(content.as_bytes())?;
    writer.write_all(AFTER_BODY.as_bytes())?;
    Ok(())
}

fn write_head(writer: &mut dyn Write, title: &str) -> io::Result<()> {
    writer.write_all(BEFORE_TITLE.as_bytes())?;
    writer.write_all(title.as_bytes())?;
    writer.write_all(BEFORE_BODY.as_bytes())?;
    Ok(())
}

const BEFORE_TITLE: &str = include_str!("../../static/before_title.html");
const BEFORE_BODY: &str = include_str!("../../static/before_body.html");
const AFTER_BODY: &str = include_str!("../../static/after_body.html");
