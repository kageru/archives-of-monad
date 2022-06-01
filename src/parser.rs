use crate::{
    data::{HasName, ObjectName},
    TRANSLATIONS,
};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::{collections::HashMap, fmt::Write};

lazy_static! {
    static ref HTML_FORMATTING_TAGS: Regex = Regex::new("</?(p|br|hr|div|span|h1|h2|h3)[^>]*>").unwrap();
    static ref APPLIED_EFFECTS_REGEX: Regex = Regex::new("(<hr ?/>\n?)?<p>Automatically applied effects:</p>\n?<ul>(.|\n)*</ul>").unwrap();
    static ref STYLE_REGEX: Regex = Regex::new(" style=\"[^\"]*\"").unwrap();
    static ref ROLL_FORMULA_REGEX: Regex = Regex::new(r"\[/b?r \{?([^}]+)\}?[\[\] ].*$").unwrap();
}

#[derive(Debug, Clone, Copy)]
// help, gib good name
enum ScopeDelimiter {
    Curly,
    Bracket,
    Angle,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
enum Token<'a> {
    Curly(&'a str),
    Bracket(&'a str),
    Html(&'a str),
    Char(char),
    String(&'a str),
    AtArea { size: i32, _type: &'a str, text: Option<&'a str> },
    AtCompendium { category: &'a str, key: &'a str, text: &'a str },
    AtLocalization { key: &'a str },
    AtCheck { _type: &'a str, dc: Option<i32>, basic: bool },
    EOF,
    ParseErr,
    ActionIcon(ActionIcon),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ActionIcon {
    Single,
    Two,
    Three,
    Free,
    Reaction,
}

/// Returns the next token and the length of the raw token in bytes
fn next_token(input: &str) -> (Token, usize) {
    match input.chars().next() {
        Some('{') => {
            let s = length_of_scope(&input[1..], ScopeDelimiter::Curly);
            (Token::Curly(&input[1..s]), s + 1)
        }
        Some('[') => {
            let s = length_of_scope(&input[1..], ScopeDelimiter::Bracket);
            (Token::Bracket(&input[1..s]), s + 1)
        }
        Some('<') => {
            let s = length_of_scope(&input[1..], ScopeDelimiter::Angle);
            if &input[1..s] == "span class=\"pf2-icon\"" {
                match input.as_bytes()[s + 1].to_ascii_lowercase() as char {
                    '1' | 'a' => (Token::ActionIcon(ActionIcon::Single), s + 9),
                    '2' | 'd' => (Token::ActionIcon(ActionIcon::Two), s + 9),
                    '3' | 't' => (Token::ActionIcon(ActionIcon::Three), s + 9),
                    'f' => (Token::ActionIcon(ActionIcon::Free), s + 9),
                    'r' => (Token::ActionIcon(ActionIcon::Reaction), s + 9),
                    _ => (Token::Html(&input[1..s]), s + 1),
                }
            } else {
                (Token::Html(&input[1..s]), s + 1)
            }
        }
        Some('@') => {
            // The layout of input here is as follows (without the spaces):
            // @SomeString[        args]
            // ^ zero     ^ arg_index  ^ arg_index + args.len()
            let arg_index = input.bytes().position(|b| b == b'[').expect("@Element without args");
            let args = &input[arg_index + 1..arg_index + length_of_scope(&input[arg_index + 1..], ScopeDelimiter::Bracket)];
            let arg_map: HashMap<_, _> = args.split('|').filter_map(|a| a.split_once(':')).collect();
            let after_args = arg_index + args.len() + 2;
            match &input[..arg_index] {
                "@Template" => {
                    let text = (input.as_bytes().get(after_args) == Some(&b'{')).then(|| {
                        let description_len = length_of_scope(&input[after_args + 1..], ScopeDelimiter::Curly);
                        &input[after_args + 1..after_args + description_len]
                    });
                    (
                        Token::AtArea {
                            size: arg_map["distance"].parse().unwrap(),
                            _type: arg_map["type"],
                            text,
                        },
                        after_args + text.map(|t| t.len() + 2).unwrap_or(0),
                    )
                }
                "@Check" => (
                    Token::AtCheck {
                        _type: arg_map["type"],
                        dc: arg_map.get("dc").and_then(|dc| dc.parse().ok()),
                        basic: *arg_map.get("basic").unwrap_or(&"false") == "true",
                    },
                    after_args,
                ),
                "@Compendium" => {
                    match args.trim_start_matches("pf2e.").trim_start_matches("Pf2e.").split_once('.') {
                        Some((category, key)) => {
                            match parse_description(&input[after_args..]) {
                                Some(text) => {
                                    let token_length = after_args + text.len() + 2; // +2 for the {}
                                    let token = Token::AtCompendium { category, key, text };
                                    (token, token_length)
                                }
                                None => {
                                    let token = Token::AtCompendium { category, key, text: key };
                                    (token, after_args)
                                }
                            }
                        }
                        None => (Token::ParseErr, after_args),
                    }
                }
                "@RollTable" => {
                    let text = parse_description(&input[after_args..]).unwrap_or_else(|| {
                        eprintln!("No description for RollTable in: {input}");
                        ""
                    });
                    let token_length = after_args + text.len() + !text.is_empty() as usize * 2;
                    (Token::String(text), token_length)
                }
                "@Localize" => (Token::AtLocalization { key: args }, after_args),
                s => {
                    eprintln!("Unknown @Formatting: {s}");
                    (Token::ParseErr, after_args)
                }
            }
        }
        Some(c) => (Token::Char(c), c.len_utf8()),
        None => (Token::EOF, 0),
    }
}

fn parse_description(input: &str) -> Option<&str> {
    input.starts_with('{').then(|| {
        let description_len = length_of_scope(&input[1..], ScopeDelimiter::Curly);
        &input[1..description_len]
    })
}

pub fn text_cleanup(mut input: &str) -> String {
    let mut s = String::with_capacity(input.len());
    loop {
        let (token, len) = next_token(input);
        // println!("{token:?}");
        match token {
            Token::EOF => break,
            Token::Char(c) => s.push(c),
            Token::String(st) => s.push_str(st),
            Token::Curly(content) => s.push_str(content),
            Token::Bracket(content) => {
                // Most rolls are formatted as `[some roll syntax]{human-readable description}`
                // or [[some roll syntax]]
                let (next, next_len) = next_token(&input[len..]);
                if let Token::Curly(annotation) = next {
                    s.push_str(annotation);
                    input = &input[next_len..];
                } else {
                    // But if they‘re not, fall back to just stripping the roll syntax and printing the formula
                    s.push_str(&ROLL_FORMULA_REGEX.replace(&content, |caps: &Captures| caps[1].to_owned()));
                }
            }
            Token::Html(content) => {
                s.push('<');
                s.push_str(&STYLE_REGEX.replace_all(content, ""));
                s.push('>');
            }
            Token::AtLocalization { key } => {
                if let Some(tl) = TRANSLATIONS.get_by_key(key) {
                    s.push_str(&text_cleanup(tl));
                } else {
                    eprintln!("Could not find translation for key {key}");
                    s.push_str(key);
                }
            }
            Token::AtCheck { _type, dc, basic } => s.push_str(&match (dc, basic) {
                (Some(dc), true) => format!("DC {dc} basic {_type}"),
                (Some(dc), false) => format!("DC {dc} {_type}"),
                (None, true) => format!("basic {_type}"),
                (None, false) => _type.to_string(),
            }),
            Token::AtCompendium { category, key: _, text } if category.to_lowercase().contains("-effects") => s.push_str(text),
            Token::AtCompendium { category, key, text } => {
                let category = match category.to_lowercase().as_str() {
                    // There are separate compendia for age-of-ashes-bestiary, abomination-vaults-bestiary, etc.
                    // We summarize these under creatures
                    cat if cat.contains("-bestiary") => "creature",
                    "feats-srd" => "feat",
                    "conditionitems" => "condition",
                    "spells-srd" => "spell",
                    "actionspf2e" => "action",
                    "action-macros" => "action", // TODO: check exhaustively if this works
                    "equipment-srd" => "item",
                    // unsure, maybe these should just both be features?
                    "ancestryfeatures" => "ancestryfeature",
                    "classfeatures" => "classfeature",
                    "hazards" => "hazard", // Should these be creatures?
                    "bestiary-ability-glossary-srd" | "bestiary-family-ability-glossary" => "creature_abilities",
                    "familiar-abilities" => "familiar_abilities",
                    "archetypes" => "archetype",
                    "backgrounds" => "background",
                    "deities" => "deity",
                    "rollable-tables" => "table",
                    "vehicles" => "creature",
                    "heritages" => "heritage",
                    "adventure-specific-actions" => "action",
                    c => unimplemented!("@Compendium category {}", c),
                };
                let item = ObjectName(key);
                write!(s, r#"<a href="/{}/{}">{}</a>"#, category, item.url_name(), text);
            }
            Token::AtArea { size, _type, text } => {
                if let Some(text) = text {
                    s.push_str(text);
                } else {
                    write!(s, "{size}-foot {_type}");
                }
            }
            Token::ParseErr => (),
            Token::ActionIcon(icon) => s.push_str(match icon {
                ActionIcon::Single => r#" <img alt="One Action" class="actionimage" src="/static/actions/OneAction.webp">"#,
                ActionIcon::Two => r#" <img alt="Two Actions" class="actionimage" src="/static/actions/TwoActions.webp">"#,
                ActionIcon::Three => r#" <img alt="Three Actions" class="actionimage" src="/static/actions/ThreeActions.webp">"#,
                ActionIcon::Free => r#" <img alt="Free Action" class="actionimage" src="/static/actions/FreeAction.webp">"#,
                ActionIcon::Reaction => r#" <img alt="Reaction" class="actionimage" src="/static/actions/Reaction.webp">"#,
            }),
        }
        input = &input[len..];
    }
    s
}

fn length_of_scope(input: &str, scope: ScopeDelimiter) -> usize {
    match (scope, input.chars().next()) {
        (ScopeDelimiter::Curly, Some('}')) | (ScopeDelimiter::Bracket, Some(']')) | (ScopeDelimiter::Angle, Some('>')) => 1,
        // Angle brackets can’t be nested
        (ScopeDelimiter::Curly, Some('{')) | (ScopeDelimiter::Bracket, Some('[')) => {
            let new_scope = length_of_scope(&input[1..], scope);
            1 + new_scope + length_of_scope(&input[new_scope + 1..], scope)
        }
        (_, Some(c)) => c.len_utf8() + length_of_scope(&input[c.len_utf8()..], scope),
        (_, None) => {
            eprintln!("Malformed roll expression, unclosed {scope:?}");
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_eq_ignore_linebreaks;
    use pretty_assertions::assert_eq;

    #[test]
    fn traverse_test() {
        let input = "additional [[/r {4d6}[precision]]]{4d6 precision damage} to frightened creatures.";
        let traversed = text_cleanup(input);
        assert_eq!(traversed, "additional 4d6 precision damage to frightened creatures.");

        let input = "Heightened +1: The damage is increased by [[/r 1d6]]";
        let traversed = text_cleanup(input);
        assert_eq!(traversed, "Heightened +1: The damage is increased by 1d6");
    }

    #[test]
    fn traverse_scope_test() {
        let input = "{some text} and some more";
        let scope_length = length_of_scope(&input[1..], ScopeDelimiter::Curly);
        assert_eq!(&input[1..scope_length], "some text");

        let input = "{some {{nested}} text} and some more";
        let scope_length = length_of_scope(&input[1..], ScopeDelimiter::Curly);
        assert_eq!(&input[1..scope_length], "some {{nested}} text");

        let input = "Deal [[/r {2d8+6}[slashing]]]{2d8+6 slashing damage} to the target";
        let start = input.chars().position(|c| c == '[').unwrap() + 1;
        let scope_length = length_of_scope(&input[start..], ScopeDelimiter::Bracket);
        assert_eq!(&input[start..start + scope_length - 1], "[/r {2d8+6}[slashing]]");
    }
    #[test]
    fn html_tag_regex_test() {
        let input = "<p>You perform rapidly, speeding up your ally.</br>";
        let expected = "You perform rapidly, speeding up your ally.";
        assert_eq!(HTML_FORMATTING_TAGS.replace_all(input, ""), expected);
    }

    #[test]
    fn inline_roll_test() {
        let input = "Freezing sleet and heavy snowfall collect on the target's feet and legs, dealing [[/r {1d4}[cold]]]{1d4 cold damage} and [[/br {5}[sad]]]{5 sad damage}";
        let expected = "Freezing sleet and heavy snowfall collect on the target's feet and legs, dealing 1d4 cold damage and 5 sad damage";
        assert_eq!(text_cleanup(input), expected);

        let input = "Heat deals [[/r {4d6}[fire]]]{4d6 fire damage}";
        assert_eq!(text_cleanup(input), "Heat deals 4d6 fire damage");

        // Without explicit description
        let input = "The creature takes [[/r {1d6}[persistent,bleed]]] @Compendium[pf2e.conditionitems.Persistent Damage]{Persistent Bleed Damage} and is @Compendium[pf2e.conditionitems.Drained]{Drained 1}.";
        let expected = r#"The creature takes 1d6 <a href="/condition/persistent_damage">Persistent Bleed Damage</a> and is <a href="/condition/drained">Drained 1</a>."#;
        assert_eq!(text_cleanup(input), expected);
    }

    #[test]
    fn legacy_inline_roll_test() {
        let input = "Freezing sleet and heavy snowfall collect on the target's feet and legs, dealing [[/r 1d4]].";
        let expected = "Freezing sleet and heavy snowfall collect on the target's feet and legs, dealing 1d4.";
        assert_eq!(text_cleanup(input), expected);

        let input = "Increase the damage to fire creatures by [[/r 2d8]].";
        let expected = "Increase the damage to fire creatures by 2d8.";
        assert_eq!(text_cleanup(input), expected);

        let input =
            "[[/r ceil(@details.level.value/2)d8 #piercing]]{Levelled} piercing damage and [[/r 123 #something]]{123 something} damage";
        let expected = "Levelled piercing damage and 123 something damage";
        assert_eq!(text_cleanup(input), expected);

        let input = "It can't use Breath Weapon again for [[/br 1d4 #rounds]]{1d4 rounds}";
        let expected = "It can't use Breath Weapon again for 1d4 rounds";
        assert_eq!(text_cleanup(input), expected);
    }

    #[test]
    fn effect_removal_test() {
        let input = "<p><strong>Frequency</strong> once per day</p>
<p><strong>Effect</strong> You gain a +10-foot status bonus to Speed for 1 minute.</p>
<p></p>
<hr />
<p>Automatically applied effects:</p>
<ul>
<li>+1 item bonus to Acrobatics checks.</li>
</ul>";
        assert_eq_ignore_linebreaks(
            &APPLIED_EFFECTS_REGEX.replace_all(input, ""),
            "<p><strong>Frequency</strong> once per day</p>
            <p><strong>Effect</strong> You gain a +10-foot status bonus to Speed for 1 minute.</p>
            <p></p>",
        );
    }

    #[test]
    fn spell_effect_replacement_test() {
        let input = "<li>
<strong>@Compendium[pf2e.spell-effects.Spell Effect: Animal Form (Ape)]{Ape}</strong>
<ul>
<li>Speed 25 feet, climb Speed 20 feet;</li>
<li><strong>Melee</strong> <span class=\"pf2-icon\">a</span> fist, <strong>Damage</strong> 2d6 bludgeoning.</li>
</ul>
</li>
<li><strong>@Compendium[pf2e.spell-effects.Spell Effect: Animal Form (Bear)]{Bear}</strong>
<ul>
<li>Speed 30 feet; </li><li><strong>Melee</strong> <span class=\"pf2-icon\">a</span> jaws, <strong>Damage</strong> 2d8 piercing;</li>
<li><strong>Melee</strong> <span class=\"pf2-icon\">a</span> claw (agile), <strong>Damage</strong> 1d8 slashing.</li>
</ul>
</li>";
        assert_eq!(text_cleanup(input), "<li>
<strong>Ape</strong>
<ul>
<li>Speed 25 feet, climb Speed 20 feet;</li>
<li><strong>Melee</strong>  <img alt=\"One Action\" class=\"actionimage\" src=\"/static/actions/OneAction.webp\"> fist, <strong>Damage</strong> 2d6 bludgeoning.</li>
</ul>
</li>
<li><strong>Bear</strong>
<ul>
<li>Speed 30 feet; </li><li><strong>Melee</strong>  <img alt=\"One Action\" class=\"actionimage\" src=\"/static/actions/OneAction.webp\"> jaws, <strong>Damage</strong> 2d8 piercing;</li>
<li><strong>Melee</strong>  <img alt=\"One Action\" class=\"actionimage\" src=\"/static/actions/OneAction.webp\"> claw (agile), <strong>Damage</strong> 1d8 slashing.</li>
</ul>
</li>");
    }

    #[test]
    fn inline_check_test() {
        let input = r#"<p>The dragon breathes a blast of flame that deals [[/r {20d6}[fire]]]{20d6 fire damage} in a @Template[type:cone|distance:60]{60-foot cone} (@Check[type:reflex|dc:42|basic:true] save).</p>\n<p>It can't use Breath Weapon again for [[/br 1d4 #Recharge Breath Weapon]]{1d4 rounds}.</p>"#;
        assert_eq!(
            text_cleanup(input),
            r#"<p>The dragon breathes a blast of flame that deals 20d6 fire damage in a 60-foot cone (DC 42 basic reflex save).</p>\n<p>It can't use Breath Weapon again for 1d4 rounds.</p>"#
        );

        let input = r#"<p>A Greater Disrupting weapon pulses with positive energy, dealing an extra 2d6 positive damage to undead On a critical hit, instead of being enfeebled 1, the undead creature must attempt a @Check[type:fortitude|dc:31|name:Greater Disrupting] save with the following effects."#;
        assert_eq!(
            text_cleanup(input),
            "<p>A Greater Disrupting weapon pulses with positive energy, dealing an extra 2d6 positive damage to undead On a critical hit, instead of being enfeebled 1, the undead creature must attempt a DC 31 fortitude save with the following effects."
        );
    }

    #[test]
    fn test_localization() {
        let input = "<p>Jaws only</p>\n<hr />\n<p>@Localize[PF2E.NPC.Abilities.Glossary.AttackOfOpportunity]</p>";
        assert_eq_ignore_linebreaks(
            &text_cleanup(input),
            "<p>Jaws only</p>
            <hr />
            <p><p><strong>Trigger</strong> A creature within the monster's reach uses a manipulate action or a move action, makes a ranged attack, or leaves a square during a move action it's using.</p>
            <hr />
            <p><strong>Effect</strong> The monster attempts a melee Strike against the triggering creature. If the attack is a critical hit and the trigger was a manipulate action, the monster disrupts that action. This Strike doesn't count toward the monster's multiple attack penalty, and its multiple attack penalty doesn't apply to this Strike.</p></p>"
        );
    }

    #[test]
    fn test_compendium_parse() {
        let input = "@Compendium[pf2e.spells-srd.Ray of Enfeeblement]{Ray of Enfeeblement}";
        let token = next_token(input);
        assert_eq!(
            token,
            (
                Token::AtCompendium {
                    category: "spells-srd",
                    key: "Ray of Enfeeblement",
                    text: "Ray of Enfeeblement"
                },
                input.len()
            )
        );

        let input = "@Compendium[pf2e.conditionitems.Friendly]{Friendly}";
        let token = next_token(input);
        assert_eq!(
            token,
            (
                Token::AtCompendium {
                    category: "conditionitems",
                    key: "Friendly",
                    text: "Friendly"
                },
                input.len()
            )
        );
    }

    #[test]
    fn test_check_parse() {
        let input = "@Check[type:will|dc:24|basic:true]";
        let token = next_token(input);
        assert_eq!(
            token,
            (
                Token::AtCheck {
                    _type: "will",
                    dc: Some(24),
                    basic: true,
                },
                input.len()
            )
        );

        let input = "@Check[type:fortitude|dc:18|traits:negative]";
        let token = next_token(input);
        assert_eq!(
            token,
            (
                Token::AtCheck {
                    _type: "fortitude",
                    dc: Some(18),
                    basic: false,
                },
                input.len()
            )
        );

        let input = "@Check[type:fortitude|dc:resolve(@actor.attributes.classDC.value)|basic:true]";
        let token = next_token(input);
        assert_eq!(
            token,
            (
                Token::AtCheck {
                    _type: "fortitude",
                    dc: None,
                    basic: true
                },
                input.len()
            )
        );
    }

    #[test]
    fn test_area_parse() {
        let input = "@Template[type:emanation|distance:30]{30 feet}";
        let token = next_token(input);
        assert_eq!(
            token,
            (
                Token::AtArea {
                    size: 30,
                    _type: "emanation",
                    text: Some("30 feet"),
                },
                input.len()
            )
        );

        let input = "@Template[type:emanation|distance:60]";
        let token = next_token(input);
        assert_eq!(
            token,
            (
                Token::AtArea {
                    size: 60,
                    _type: "emanation",
                    text: None,
                },
                input.len()
            )
        );
    }

    #[test]
    fn test_localize_parse() {
        let input = "@Localize[PF2E.NPC.Abilities.Glossary.AttackOfOpportunity]";
        let token = next_token(input);
        assert_eq!(
            token,
            (
                Token::AtLocalization {
                    key: "PF2E.NPC.Abilities.Glossary.AttackOfOpportunity"
                },
                input.len()
            )
        );
    }

    #[test]
    fn test_simple_roll_tokenizer() {
        let mut input = "dealing [[/r {4d6}[fire]]]{4d6 fire damage} for [[/r 1d4]] rounds";
        let mut tokens = Vec::new();
        while !input.is_empty() {
            let (token, offset) = next_token(input);
            tokens.push(token);
            input = &input[offset..];
        }
        assert_eq!(next_token(input), (Token::EOF, 0));
        assert_eq!(
            tokens,
            vec![
                Token::Char('d',),
                Token::Char('e',),
                Token::Char('a',),
                Token::Char('l',),
                Token::Char('i',),
                Token::Char('n',),
                Token::Char('g',),
                Token::Char(' ',),
                Token::Bracket("[/r {4d6}[fire]]",),
                Token::Curly("4d6 fire damage",),
                Token::Char(' ',),
                Token::Char('f',),
                Token::Char('o',),
                Token::Char('r',),
                Token::Char(' ',),
                Token::Bracket("[/r 1d4]",),
                Token::Char(' ',),
                Token::Char('r',),
                Token::Char('o',),
                Token::Char('u',),
                Token::Char('n',),
                Token::Char('d',),
                Token::Char('s',),
            ]
        );
    }

    #[test]
    fn test_compendium_reference() {
        let input = "<p>As a anadi, you gain the @Compendium[pf2e.actionspf2e.Change Shape (Anadi)]{Change Shape (Anadi)} ability.</p>";
        let expected = r#"<p>As a anadi, you gain the <a href="/action/change_shape_anadi">Change Shape (Anadi)</a> ability.</p>"#;
        assert_eq!(text_cleanup(input), expected);

        let input = "Some forms of magical darkness, such as a 4th-level <em>@Compendium[pf2e.spells-srd.Darkness]{Darkness}</em> spell, block normal darkvision. A monster with @Compendium[pf2e.bestiary-ability-glossary-srd.Greater Darkvision]{Greater Darkvision}, however, can see through even these forms of magical darkness.";
        let expected = r#"Some forms of magical darkness, such as a 4th-level <em><a href="/spell/darkness">Darkness</a></em> spell, block normal darkvision. A monster with <a href="/creature_abilities/greater_darkvision">Greater Darkvision</a>, however, can see through even these forms of magical darkness."#;
        assert_eq!(text_cleanup(input), expected);
    }
}
