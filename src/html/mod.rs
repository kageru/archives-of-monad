pub mod feats;
pub mod spells;

#[cfg(test)]
mod tests {
    use crate::data::backgrounds::Background;
    use crate::data::deities::Deity;
    use crate::data::feats::Feat;
    use crate::data::spells::Spell;
    use crate::data::traits::read_trait_descriptions;
    use crate::html::feats::FeatTemplate;
    use crate::html::spells::SpellTemplate;
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

    #[test]
    fn test_spell_template() {
        let f = std::fs::File::open("tests/data/spells/heal.json").expect("File missing");
        let reader = BufReader::new(f);
        let heal: Spell = serde_json::from_reader(reader).expect("Deserialization failed");

        let descriptions = read_trait_descriptions("tests/data/en.json");

        let heal = SpellTemplate::new(heal, &descriptions);
        let expected = include_str!("../../tests/html/heal.html");
        assert_eq!(heal.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_spell_template2() {
        let f = std::fs::File::open("tests/data/spells/resurrect.json").expect("File missing");
        let reader = BufReader::new(f);
        let res: Spell = serde_json::from_reader(reader).expect("Deserialization failed");

        let descriptions = read_trait_descriptions("tests/data/en.json");

        let res = SpellTemplate::new(res, &descriptions);
        let expected = include_str!("../../tests/html/resurrect.html");
        assert_eq!(res.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_deity_template() {
        let f = std::fs::File::open("tests/data/deities/asmodeus.json").expect("File missing");
        let reader = BufReader::new(f);
        let asmodeus: Deity = serde_json::from_reader(reader).expect("Deserialization failed");
        let expected = include_str!("../../tests/html/asmodeus.html");
        assert_eq!(asmodeus.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_background_template() {
        let raw = std::fs::read_to_string("tests/data/backgrounds/field-medic.json").expect("File missing");
        let field_medic: Background = serde_json::from_str(&raw).expect("Deserialization of background failed");
        let expected = include_str!("../../tests/html/field_medic.html");
        assert_eq!(field_medic.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }
}
