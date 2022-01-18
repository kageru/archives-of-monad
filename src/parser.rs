#[derive(Debug, Clone, Copy)]
// help, gib good name
enum ScopeDelimiter {
    Curly,
    Bracket,
}

enum Scope<'a> {
    Curly(&'a str),
    Bracket(&'a str),
    Char(char),
    EOL,
}

fn next_scope(input: &str) -> Scope {
    match input.chars().next() {
        Some('{') => {
            let s = traverse_scope(&input[1..], ScopeDelimiter::Curly);
            Scope::Curly(&input[1..s])
        }
        Some('[') => {
            let s = traverse_scope(&input[1..], ScopeDelimiter::Bracket);
            Scope::Bracket(&input[1..s])
        }
        Some(c) => Scope::Char(c),
        None => Scope::EOL,
    }
}

fn traverse_string(mut input: &str) -> String {
    let mut s = String::with_capacity(input.len());
    loop {
        match next_scope(input) {
            Scope::Char(c) => {
                s.push(c);
                input = &input[1..];
            }
            Scope::EOL => break,
            Scope::Curly(content) => {
                s.push_str(content);
                input = &input[2 + content.len()..];
            }
            Scope::Bracket(content) => {
                input = &input[2 + content.len()..];
                // Most rolls are formatted as `[some roll syntax]{human-readable description}`
                if let Scope::Curly(annotation) = next_scope(input) {
                    s.push_str(annotation);
                    input = &input[2 + annotation.len()..];
                } else {
                    // But if they‘re not, fall back to just stripping the roll syntax
                    // and printing the formula
                    s.push_str(content.trim_start_matches("[/r ").trim_start_matches("[/br ").trim_end_matches("]"));
                }
            }
        }
    }
    s
}

fn traverse_scope(input: &str, scope: ScopeDelimiter) -> usize {
    match (scope, input.chars().next().expect("Expression is not well-formed")) {
        (ScopeDelimiter::Curly, '}') | (ScopeDelimiter::Bracket, ']') => 1,
        (ScopeDelimiter::Curly, '{') | (ScopeDelimiter::Bracket, '[') => {
            let new_scope = traverse_scope(&input[1..], scope);
            1 + new_scope + traverse_scope(&input[new_scope + 1..], scope)
        }
        _ => 1 + traverse_scope(&input[1..], scope),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traverse_test() {
        let input = "additional [[/r {4d6}[precision]]]{4d6 precision damage} to frightened creatures.";
        let traversed = traverse_string(input);
        assert_eq!(traversed, "additional 4d6 precision damage to frightened creatures.");

        let input = "Heightened +1: The damage is increased by [[/r 1d6]]";
        let traversed = traverse_string(input);
        assert_eq!(traversed, "Heightened +1: The damage is increased by 1d6");
    }

    #[test]
    fn traverse_scope_test() {
        let input = "{some text} and some more";
        let scope_length = traverse_scope(&input[1..], ScopeDelimiter::Curly);
        assert_eq!(&input[1..scope_length], "some text");

        let input = "{some {{nested}} text} and some more";
        let scope_length = traverse_scope(&input[1..], ScopeDelimiter::Curly);
        assert_eq!(&input[1..scope_length], "some {{nested}} text");

        let input = "Deal [[/r {2d8+6}[slashing]]]{2d8+6 slashing damage} to the target";
        let start = input.chars().position(|c| c == '[').unwrap() + 1;
        let scope_length = traverse_scope(&input[start..], ScopeDelimiter::Bracket);
        assert_eq!(&input[start..start + scope_length - 1], "[/r {2d8+6}[slashing]]");
    }
}
