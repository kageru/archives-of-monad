use crate::{
    data::{HasName, boons_and_curses::BoonOrCurse, feat_type::FeatType},
    html::{HtmlPage, Template},
};
use std::borrow::Cow;

impl Template<()> for BoonOrCurse {
    fn render(&self, _: ()) -> Cow<'_, str> {
        Cow::Owned(format!(
            "<h1><a href=\"/boon_curse/{}\">{}</a><span class=\"type\">{}</span></h1><hr/>{}",
            self.url_name(),
            self.name,
            self.category(),
            self.description,
        ))
    }

    fn render_index(elements: &[(Self, HtmlPage)]) -> String {
        let mut index = String::with_capacity(50_000);
        index.push_str("<div id=\"gridlist\">");
        for (boon_or_curse, _) in elements {
            index.push_str(&format!(
                "<span><a href=\"{}\">{} ({})</a></span>",
                boon_or_curse.url_name(),
                boon_or_curse.name(),
                boon_or_curse.category(),
            ));
        }
        index.push_str("</div>");
        index
    }

    fn category(&self) -> Cow<'_, str> {
        Cow::Borrowed(match self.feat_type {
            FeatType::Boon => "Boon",
            FeatType::Curse => "Curse",
            other => unreachable!("Unexpected boon/curse feat type: {:?}", other),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_eq_ignore_linebreaks, read_test_file};

    #[test]
    fn test_boon_template() {
        let boon: BoonOrCurse =
            serde_json::from_str(&read_test_file("boons-and-curses/asmodeus-major-boon.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&boon.render(()), include_str!("../../tests/html/asmodeus_major_boon.html"));
    }

    #[test]
    fn test_curse_template() {
        let curse: BoonOrCurse =
            serde_json::from_str(&read_test_file("boons-and-curses/cayden-cailean-minor-curse.json")).expect("Deserialization failed");
        assert_eq_ignore_linebreaks(&curse.render(()), include_str!("../../tests/html/cayden_cailean_minor_curse.html"));
    }
}
