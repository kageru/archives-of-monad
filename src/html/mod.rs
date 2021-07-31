pub mod feats;

#[cfg(test)]
mod tests {
    use crate::data::feats::Feat;
    use crate::data::traits::read_trait_descriptions;
    use crate::html::feats::FeatTemplate;
    use askama::Template;
    use itertools::Itertools;
    use std::io::BufReader;

    #[test]
    fn test_feat_template() {
        let f = std::fs::File::open("tests/data/feats/sever-space.json").expect("File missing");
        let reader = BufReader::new(f);
        let feat: Feat = serde_json::from_reader(reader).expect("Deserialization failed");

        let descriptions = read_trait_descriptions("tests/data/en.json");

        let feat = FeatTemplate::new(feat, &descriptions);
        let expected = include_str!("../../tests/html/sever_space.html");
        assert_eq!(feat.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }
}
