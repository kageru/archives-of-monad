#[cfg(test)]
mod tests {
    use crate::data::feats::Feat;
    use askama::Template;
    use std::io::BufReader;

    #[test]
    fn test_feat_template() {
        let f = std::fs::File::open("tests/data/feats/sever-space.json").expect("File missing");
        let reader = BufReader::new(f);
        let feat: Feat = serde_json::from_reader(reader).expect("Deserialization failed");
        let expected = include_str!("../../tests/html/sever_space.html");
        feat.render()
            .unwrap()
            .lines()
            .zip(expected.lines())
            .for_each(|(actual, expected)| assert_eq!(expected, actual));
    }
}
