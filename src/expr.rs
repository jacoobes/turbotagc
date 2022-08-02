use std::ops::Range;

#[derive(Debug)]
pub enum Expr {
    Var(String),
    Call(String, Vec<Expr>),
    Bool(bool),
    Def {
        name: String,
        rhs: Box<Expr>,
        then: Box<Expr>,
    },
    Fn {
        name: String,
        args: Vec<String>,
        body: Box<Expr>,
        then: Box<Expr>,
    },
    FillIn(Box<Expr>),
    Pipeable {
        expr : Box<Expr>,
        chain : Vec<Expr>
    },
    Template(String)
}