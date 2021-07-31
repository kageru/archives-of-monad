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
        assert_eq!(
            "<h2>Sever Space</h2>
<p>(Feat 20)</p>
<p><p><strong>Requirements</strong> You are wielding a weapon that deals slashing damage or have an unarmed Strike that deals slashing damage.</p>
<hr />
<p>You destroy the space between you and your targets, allowing you to strike with your melee weapons at great range. Make a melee Strike with the required weapon or unarmed attack. The attack gains an 80-foot reach for this Strike.</p>
<p>After the Strike, regardless of whether it succeeded, the world rushes to fill the space you destroyed, bringing you and the target adjacent to each other. You can choose to teleport to the closest space adjacent to the target or to attempt to teleport the target adjacent to you. If you choose the target, they can negate the teleportation if they succeed at a Fortitude save against your class DC.</p></p>",
            feat.render().unwrap()
        );
    }
}
