use std::ops::Range;

#[derive(Debug)]
pub enum Expr {
    Var(String),
    Call(String, Vec<Expr>),
    Bool(bool),
    FillIn(Vec<Expr>),
    Pipeable {
        expr : Box<Expr>,
        chain : Vec<Expr>
    },
    Template(String),
    Whitespace(String)
}