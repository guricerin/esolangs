#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ast {
    Stmts(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Expr(Expr),
    NumOut(Expr),
    CharOut(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Var(Variable),
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

    pub fn int(i: i64) -> Self {
        Self::Var(Variable::Int(i))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Variable {
    Assign { var: char, expr: Box<Expr> },
    Var(char),
    Int(i64),
}

impl Variable {
    pub fn assign(var: char, expr: Expr) -> Self {
        Self::Assign {
            var: var,
            expr: Box::new(expr),
        }
    }
}
