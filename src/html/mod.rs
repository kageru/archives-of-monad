pub mod feats;
pub mod spells;

#[cfg(test)]
mod tests {
    use crate::data::archetypes::Archetype;
    use crate::data::backgrounds::Background;
    use crate::data::conditions::Condition;
    use crate::data::deities::Deity;
    use crate::data::feats::Feat;
    use crate::data::spells::Spell;
    use crate::data::traits::read_trait_descriptions;
    use crate::html::feats::FeatTemplate;
    use crate::html::spells::SpellTemplate;
    use crate::replace_references;
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
        let field_medic = Background {
            description: replace_references(&field_medic.description).to_string(),
            ..field_medic
        };
        let expected = include_str!("../../tests/html/field_medic.html");
        assert_eq!(field_medic.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_background_template_haunted() {
        let raw = std::fs::read_to_string("tests/data/backgrounds/haunted.json").expect("File missing");
        let haunted: Background = serde_json::from_str(&raw).expect("Deserialization of background failed");
        let haunted = Background {
            description: replace_references(&haunted.description).to_string(),
            ..haunted
        };
        let expected = include_str!("../../tests/html/haunted.html");
        assert_eq!(haunted.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_archetype_template() {
        let raw = std::fs::read_to_string("tests/data/archetypes/assassin.json").expect("File missing");
        let assassin: Archetype = serde_json::from_str(&raw).expect("Deserialization of background failed");
        let assassin = Archetype {
            content: replace_references(&assassin.content).to_string(),
            ..assassin
        };
        let expected = include_str!("../../tests/html/assassin.html");
        assert_eq!(assassin.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_condition_template() {
        let f = std::fs::File::open("tests/data/conditions/blinded.json").expect("File missing");
        let reader = BufReader::new(f);
        let blinded: Condition = serde_json::from_reader(reader).expect("Deserialization failed");
        let expected = include_str!("../../tests/html/blinded.html");
        assert_eq!(blinded.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }
}
