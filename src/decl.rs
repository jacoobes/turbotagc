use crate::Expr;
#[derive(Debug)]
pub enum Decl {
    Def {
        name: String,
        rhs: Vec<Expr>,
    }
}