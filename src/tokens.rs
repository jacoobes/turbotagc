use std::borrow::Cow;
use logos::{ Lexer, Logos };

#[derive(Logos, Debug, PartialEq, Clone, Hash, Eq)]
pub enum Token {
    // Tokens can be literal strings, of any length.
    #[token("$")]
    Dollar,
    #[token("|")]
    Pipe,
    #[regex("[a-zA-Z0-9]+", parse_str)]
    Ident(String),
    #[regex(r"\$[a-zA-Z]+", parse_str)]
    Template(String),
    #[token("{{")]
    LLBrace,
    #[token("}}")]
    RRBrace,
    #[token("=")]
    Equal,
    #[token("??")]
    Or,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LSquare,
    #[token("]")]
    RSquare,
    #[token(">")]
    RArrow,
    #[regex(r"true|false", parse_bool)]
    Bool(bool),
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    Error,
}

fn parse_bool(lex: &mut Lexer<Token>) -> Option<bool> {
    let slice: &str = lex.slice();
    slice.parse().ok()
}

fn parse_str(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    Some(slice.to_string())
}

