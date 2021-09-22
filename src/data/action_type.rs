use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    Action,
    Reaction,
    Passive,
    Free,
}

impl ActionType {
    pub fn img(&self, actions: &Option<i32>) -> &str {
        match *self {
            ActionType::Passive => r#"<img alt="Passive" class="actionimage" src="/static/actions/Passive.webp">"#,
            ActionType::Free => r#"<img alt="Free Action" class="actionimage" src="/static/actions/FreeAction.webp">"#,
            ActionType::Reaction => r#"<img alt="Reaction" class="actionimage" src="/static/actions/Reaction.webp">"#,
            ActionType::Action => match actions {
                Some(1) => r#"<img alt="One Action" class="actionimage" src="/static/actions/OneAction.webp">"#,
                Some(2) => r#"<img alt="Two Actions" class="actionimage" src="/static/actions/TwoActions.webp">"#,
                Some(3) => r#"<img alt="Three Actions" class="actionimage" src="/static/actions/ThreeActions.webp">"#,
                None => "",
                _ => unreachable!(),
            },
        }
    }
}
