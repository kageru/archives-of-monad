use std::borrow::Cow;

pub mod actions;
pub mod conditions;
pub mod feats;
pub mod spells;
pub mod deities;

trait Template {
    fn render(&self) -> Cow<'_, str>;
}

#[cfg(test)]
mod tests {
    use crate::data::{actions::Action, archetypes::Archetype, backgrounds::Background, feats::Feat, spells::Spell};
    use crate::html::{actions::ActionTemplate, feats::FeatTemplate, spells::SpellTemplate};
    use crate::tests::read_test_file;
    use crate::tests::DESCRIPTIONS;
    use askama::Template;
    use itertools::Itertools;

    #[test]
    fn test_feat_template() {
        let feat: Feat = serde_json::from_str(&read_test_file("feats.db/sever-space.json")).expect("Deserialization failed");
        let feat = FeatTemplate::new(feat, &DESCRIPTIONS);
        let expected = include_str!("../../tests/html/sever_space.html");
        assert_eq!(feat.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_spell_template() {
        let heal: Spell = serde_json::from_str(&read_test_file("spells.db/heal.json")).expect("Deserialization failed");
        let heal = SpellTemplate::new(heal, &DESCRIPTIONS);
        let expected = include_str!("../../tests/html/heal.html");
        assert_eq!(heal.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_spell_template2() {
        let res: Spell = serde_json::from_str(&read_test_file("spells.db/resurrect.json")).expect("Deserialization failed");
        let res = SpellTemplate::new(res, &DESCRIPTIONS);
        let expected = include_str!("../../tests/html/resurrect.html");
        assert_eq!(res.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_background_template() {
        let field_medic: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/field-medic.json")).expect("Deserialization of background failed");
        let expected = include_str!("../../tests/html/field_medic.html");
        assert_eq!(field_medic.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_background_template_haunted() {
        let haunted: Background =
            serde_json::from_str(&read_test_file("backgrounds.db/haunted.json")).expect("Deserialization of background failed");
        let expected = include_str!("../../tests/html/haunted.html");
        assert_eq!(haunted.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_archetype_template() {
        let assassin: Archetype =
            serde_json::from_str(&read_test_file("archetypes.db/assassin.json")).expect("Deserialization of background failed");
        let expected = include_str!("../../tests/html/assassin.html");
        assert_eq!(assassin.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }

    #[test]
    fn test_action_template() {
        let aid: Action = serde_json::from_str(&read_test_file("actions.db/aid.json")).expect("Deserialization failed");
        let aid = ActionTemplate::new(aid, &DESCRIPTIONS);
        let expected = include_str!("../../tests/html/aid.html");
        assert_eq!(aid.render().unwrap().lines().join("\n"), expected.lines().join("\n"));
    }
}
