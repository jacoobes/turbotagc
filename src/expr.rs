
#[derive(Debug, Clone)]
pub enum Expr {
    Var(String),
    Bool(bool),
    FillIn(Box<Expr>),
    Pipeable(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Template(String),
    Whitespace(String)
}