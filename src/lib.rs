extern crate core;


use chumsky::prelude::*;
use chumsky::Stream;
use logos::{Logos, Span};
use logos::Lexer;
use wasm_bindgen::prelude::*;

use crate::expr::Expr;
use crate::tokens::Token;

mod utils;
mod tokens;
mod expr;
mod lex_tostream;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(str: &str);
}


#[wasm_bindgen]
pub fn compile(content : &str)  {
    let mut lexer : Lexer<Token> = Token::lexer(content);
    let stream = Stream::from_iter(Span::new((), 0usize..lexer.source().len()), lexer.spanned());
    let formatted = format!("{:?}", expr_parser().parse(stream));
    log(formatted.as_ref() );
}
fn expr_parser<'source>() -> impl Parser<Token, Vec<Expr>, Error = Simple<Token>> {
    use chumsky::prelude::*;

    recursive(|expr| {
        let prims = select! {
            Token::Bool(b) => Expr::Bool(b),
            Token::Ident(i) => Expr::Var(i),
            Token::Template(i) => Expr::Template(i),
            Token::Whitespace(i) => Expr::Whitespace(i),
        }.labelled("prims");

        let pipeable=  just(Token::LSquare)
            .ignore_then(
                expr.clone()
                    .then_ignore(just(Token::RArrow))
                    .then(
                     expr.clone().
                        separated_by(
                            just(Token::Pipe)
                        )
                        .allow_leading()
                        .allow_trailing()
                    )
            )
            .then_ignore(just(Token::RSquare))
            .map(| (expr, chain) : (Expr, Vec<Expr>)   | {
                Expr::Pipeable { expr: Box::new(expr), chain, }
            });

        let fill_ins = just(Token::LLBrace)
                .ignore_then(expr.clone().repeated())
                .then_ignore(just(Token::RRBrace))
                .map(|t : Vec<Expr>| {
                    Expr::FillIn(t)
                }).labelled("fill_in");
        let atom =
            prims
              .or(pipeable)
              .or(fill_ins);
        atom
    }).repeated().then_ignore(end())

}
