#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ast {
    Int(i64),
    BinOp { op: BinOp, l: Box<Ast>, r: Box<Ast> },
}

impl Ast {
    pub fn binop(op: BinOp, l: Ast, r: Ast) -> Self {
        Self::BinOp {
            op: op,
            l: Box::new(l),
            r: Box::new(r),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
