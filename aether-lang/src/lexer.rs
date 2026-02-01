use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")] // Skip whitespace
pub enum Token {
    // Keywords
    #[token("agent")]
    Agent,
    #[token("state")]
    State,
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("return")]
    Return,
    #[token("thinking")]
    Thinking,
    #[token("@compute")]
    ComputeDecorator,

    // Symbols
    #[token("{")]
    BraceOpen,
    #[token("}")]
    BraceClose,
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,
    #[token("=")]
    Equals,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token(";")]
    Semi,

    // Literals
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    StringLit(String),
    
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    IntLit(i64),

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    // Error
    #[regex(r"//.*", logos::skip)] // Comments
    Error,
}
