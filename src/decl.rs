use crate::Expr;
#[derive(Debug)]
pub enum Decl {
    Fn {
        name: String,
        args: Vec<String>,
        body: Vec<Expr>,
        then: Vec<Expr>,
    },

    Def {
        name: String,
        rhs: Vec<Expr>,
    },

}