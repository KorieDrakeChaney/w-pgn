use crate::Nag;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftBracket,
    RightBracket,
    LeftParenthesis,
    RightParenthesis,

    Symbol(String),
    String(String),

    Termination(String),

    Comment(String),
    Nag(Nag),
}
