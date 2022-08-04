use std::borrow::BorrowMut;
use logos::{Lexer, Logos, Source};

#[derive(Logos, Debug, PartialEq, Clone, Hash, Eq)]
pub enum Token {
    // Tokens can be literal strings, of any length.
    #[token("|")]
    Pipe,
    #[regex("[a-zA-Z0-9]+", parse_str)]
    Ident(String),
    #[regex(r"\$[a-zA-Z]+", parse_template)]
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
    #[token("def")]
    Def,
    #[regex(r"true|false", parse_bool)]
    Bool(bool),
    #[token("fn")]
    Function,
    #[token("start")]
    Start,
    #[token("end")]
    End,
    #[token(",")]
    Comma,
    #[regex(r"\\s|\\n|\\r|\\t|\\b|\\f|\\r|", parse_str)]
    Whitespace(String),
    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
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

fn parse_template(lex: &mut Lexer<Token>) -> Option<String> {
    let slice: &str = lex.slice();
    Some(slice.slice(1..slice.len()).unwrap().to_string())
}
