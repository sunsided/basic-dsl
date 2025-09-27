// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

//! Abstract Syntax Tree definitions for the BASIC DSL

#[derive(Clone)]
pub enum Stmt {
    Label(i32),
    Let(String, Expr),
    Print(Expr),
    Goto(i32),
    IfGoto {
        lhs: Expr,
        op: Cmp,
        rhs: Expr,
        target: i32,
    },
    End,
}

#[derive(Clone)]
pub enum Expr {
    Num(i64),
    Str(String),
    Var(String),
    Bin {
        lhs: Box<Expr>,
        op: Bin,
        rhs: Box<Expr>,
    },
}

#[derive(Clone, Copy)]
pub enum Bin {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Copy)]
pub enum Cmp {
    Lt,
    Le,
    Eq,
    Ge,
    Gt,
}

/// Used during lowering - represents atoms in the expression pool
#[derive(Clone)]
pub enum Atom {
    Imm(i64),
    Str(String),
    Var(usize),
    Bin(Bin),
}
