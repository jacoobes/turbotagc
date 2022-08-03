
#[derive(Debug, Clone)]
pub enum Expr {
    Var(String),
    Bool(bool),
    FillIn(Vec<Expr>),
    Pipeable {
        expr : Box<Expr>,
        chain : Vec<Expr>
    },
    Or(Box<Expr>),
    Template(String),
    Whitespace(String)
}