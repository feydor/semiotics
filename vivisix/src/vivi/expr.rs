/** Expr - mathematical expressions */

pub struct Expr {
    op: Op,
    args: Vec<Expr>,
}

enum Op {
    Plus,
    Minus,
    Star,
    Slash
}

impl Expr {
    fn make() -> Self {
        Self{ op: Op::Plus, args: [].to_vec() }
    }
}
