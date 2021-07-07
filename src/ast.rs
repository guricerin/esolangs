#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ast {
    Stmt { stmt: Stmt, expr: Expr },
    Expr(Expr),
}

impl Ast {
    pub fn stmt(stmt: Stmt, expr: Expr) -> Self {
        Self::Stmt { stmt, expr }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    NumOut,
    CharOut,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Int(i64),
    BinOp {
        op: BinOp,
        l: Box<Expr>,
        r: Box<Expr>,
    },
}

impl Expr {
    pub fn binop(op: BinOp, l: Expr, r: Expr) -> Self {
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
