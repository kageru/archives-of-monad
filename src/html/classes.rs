use super::Template;
use crate::data::{classes::Class, traits::TraitDescriptions, HasName};
use std::borrow::Cow;

impl Template<&TraitDescriptions> for Class {
    fn render(&self, _: &TraitDescriptions) -> Cow<'_, str> {
        Cow::Borrowed(&self.description)
    }

    fn render_index(elements: &[Self]) -> String {
        let mut page = String::with_capacity(1000);
        page.push_str("<h1>Classes</h1><hr/><div id=\"list\">");
        for class in elements {
            page.push_str(&format!("<h2><a href=\"{}\">{}</a></h2><br/>", class.url_name(), class.name));
        }
        page.push_str("</div>");
        page
    }
}
