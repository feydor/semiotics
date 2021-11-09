/** Expr - mathematical expressions */

pub struct Expr {
    op: Op,
    args: Vec<Expr>,
}

pub enum Op {
    Plus,
    Minus,
    Star,
    Slash
}
