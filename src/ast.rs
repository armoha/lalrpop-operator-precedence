use std::fmt::{Debug, Error, Formatter};

pub enum Expr {
    Number(i32),
    UnOp(UnOpcode, Box<Expr>),
    BinOp(Box<Expr>, BinOpcode, Box<Expr>),
    Error,
}

#[derive(Copy, Clone)]
pub enum UnOpcode {
    Negate,
    Not,
}

#[derive(Copy, Clone)]
pub enum BinOpcode {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    BitLsh,
    BitRsh,
    LogicAnd,
    LogicOr,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            UnOp(op, ref r) => write!(fmt, "({:?} {:?})", op, r),
            BinOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for UnOpcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::UnOpcode::*;
        match *self {
            Negate => write!(fmt, "-"),
            Not => write!(fmt, "!"),
        }
    }
}

impl Debug for BinOpcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::BinOpcode::*;
        match *self {
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Mod => write!(fmt, "%"),
            BitAnd => write!(fmt, "&"),
            BitOr => write!(fmt, "|"),
            BitXor => write!(fmt, "^"),
            BitLsh => write!(fmt, "<<"),
            BitRsh => write!(fmt, ">>"),
            LogicAnd => write!(fmt, "&&"),
            LogicOr => write!(fmt, "||"),
            Equal => write!(fmt, "=="),
            NotEqual => write!(fmt, "!="),
            LessThan => write!(fmt, "<"),
            GreaterThan => write!(fmt, ">"),
            LessThanEqual => write!(fmt, "<="),
            GreaterThanEqual => write!(fmt, ">="),
        }
    }
}
