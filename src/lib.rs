extern crate core;


use chumsky::prelude::*;
use chumsky::Stream;
use logos::{Logos, Span};
use logos::Lexer;
use wasm_bindgen::prelude::*;

use crate::decl::Decl;
use crate::expr::Expr;
use crate::tokens::Token;

mod utils;
mod tokens;
mod expr;
mod lex_tostream;
mod decl;

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
pub fn compile(content: &str) {
    let mut lexer: Lexer<Token> = Token::lexer(content);
    let stream = Stream::from_iter(Span::new((), 0usize..lexer.source().len()), lexer.spanned());

    log(&format!("{:?}", parser().parse(stream)))
}

fn parser() -> impl Parser<Token, (Vec<Decl>, Vec<Expr>), Error=Simple<Token>> {
    decl_parser()
        .then(
            expr_parser()
                .delimited_by(
                    just(Token::Start), just(Token::End),
                )
        )
        .then_ignore(end())
}

fn expr_parser() -> impl Parser<Token, Vec<Expr>, Error=Simple<Token>> {
    use chumsky::prelude::*;

    recursive(|expr| {
        let prims = select! {
            Token::Bool(b) => Expr::Bool(b),
            Token::Ident(i) => Expr::Var(i),
            Token::Template(i) => Expr::Template(i),
            Token::Whitespace(i) => Expr::Whitespace(i),
        }.labelled("prims");
        let atoms = prims.or(
            expr.clone().delimited_by(just(Token::LParen), just(Token::RParen)));
        let pipeable = prims
            .clone()
            .then_ignore(just(Token::RArrow))
            .then(
                prims
                    .then_ignore(just(Token::Pipe))
                    .repeated()
            )
            .foldl(|v, s : Expr |
                Expr::Pipeable(Box::new(v), Box::new(s))
            ).then_ignore(just(Token::End));

        let fill_ins =
            expr.clone()
                .delimited_by(
            just(Token::LLBrace), just(Token::RRBrace)
        ).map(|e: Expr | Expr::FillIn(Box::new(e)));

        // let or_ =
        //     fill_ins.clone()
        //         .then(just(Token::Or))
        //         .to(Expr::Or  as fn(_, _) -> _)
        //         .then(
        //             fill_ins.repeated()
        //                 .foldl(|lhs, rhs| {
        //                     Expr::Or(Box::new(lhs.0), Box::new(rhs.1))
        //                 })
        //         );

        fill_ins
            .or(pipeable)
            .or(atoms)
    }).repeated()
}


fn decl_parser() -> impl Parser<Token, Vec<Decl>, Error=Simple<Token>> {
    let idents = select! { Token::Ident(i) => i.to_string() };
    let defs = just(Token::Def)
        .ignore_then(idents)
        .then_ignore(just(Token::Equal))
        .then(expr_parser())
        .map(|s| {
            Decl::Def { name: s.0, rhs: s.1 }
        })
        .then_ignore(just(Token::End));
    defs.repeated()
}