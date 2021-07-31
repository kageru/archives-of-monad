#[cfg(test)]
mod tests {
    use crate::data::feats::{Feat, FeatTemplate};
    use crate::data::spells::Spell;
    use crate::data::traits::read_trait_descriptions;
    use askama::Template;
    use std::io::BufReader;
    use itertools::Itertools;

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

    /* #[test]
    fn test_spell_template() {
        let f = std::fs::File::open("tests/data/spells/heal.json").expect("File missing");
        let reader = BufReader::new(f);
        let heal: Spell = serde_json::from_reader(reader).expect("Deserialization failed");
        let expected = include_str!("../../tests/html/heal.html");
        heal.render()
            .unwrap()
            .lines()
            .zip(expected.lines())
            .for_each(|(actual, expected)| assert_eq!(expected, actual));
    }*/
}
