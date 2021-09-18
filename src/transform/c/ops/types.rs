use super::super::{
    Let,
    types::TypeID
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]

/// For single operators
pub enum Fix {
    /// <op> a (e.g. -a)
    Prefix,

    /// a <op> (e.g. a-)
    Postfix
}

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Order {
    /// a <op> b <op> c == (a <op> b) <op> c
    Left,

    /// a <op> b <op> c == a <op> (b <op> c)
    Right
}

pub struct Op {
    pub operands: Vec <TypeID>,
    pub ret: TypeID,
    pub parts: Vec <String>,
    pub fix: Fix,
    pub order: Order,
    pub priority: u8,
    pub make: fn(&Vec <&Let>) -> String
}

impl Op {
    pub fn ops() -> &'static mut Vec <Op> {
        static mut OPS: Vec <Op> = Vec::new();

        unsafe { &mut OPS }
    }

    pub fn add(op: Op) {
        let mut i = 0;
        while i < Self::ops().len() {
            if Self::ops()[i].priority < op.priority {
                Self::ops().insert(i, op);
                return
            }
            i += 1
        }
        Self::ops().push(op)

    }
}

#[derive(Debug, Clone)]
pub enum Operand <'a> {
    Let(&'a Let),
    Expr(Let)
}

impl <'a> Operand <'a> {
    pub fn ty(&self) -> TypeID {
        match self {
            Self::Let(x) => x.ty,
            Self::Expr(x) => x.ty
        }
    }

    pub fn as_let(&self) -> &Let {
        match self {
            Self::Let(x) => x,
            Self::Expr(x) => x
        }
    }
}

#[derive(Debug, Clone)]
pub struct Operator {
    pub data: String,
    pub fix: Fix
}
