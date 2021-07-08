use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ast {
    Stmts(Stmts),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetVal {
    Int(i64),
    Void,
}

impl RetVal {
    pub fn to_i(&self) -> Result<i64> {
        match self {
            Self::Int(i) => Ok(*i),
            Self::Void => Err(anyhow::anyhow!("the retrun value type is void.")),
        }
    }
}

pub type Stmts = Vec<Stmt>;

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
    If {
        cond: Box<Expr>,
        conseq: Box<Stmts>,
        alt: Option<Box<Stmts>>,
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

    pub fn if_alt(cond: Expr, conseq: Stmts, alt: Stmts) -> Self {
        Self::If {
            cond: Box::new(cond),
            conseq: Box::new(conseq),
            alt: Some(Box::new(alt)),
        }
    }

    pub fn if_without_alt(cond: Expr, conseq: Stmts) -> Self {
        Self::If {
            cond: Box::new(cond),
            conseq: Box::new(conseq),
            alt: None,
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
