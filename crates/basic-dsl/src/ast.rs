// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

//! Abstract Syntax Tree definitions for the BASIC DSL

/// Represents an expression in a PRINT statement with its separator
#[derive(Clone)]
pub struct PrintItem {
    pub expr: Expr,
    pub separator: PrintSeparator,
}

/// Type of separator after a PRINT expression
#[derive(Clone, Copy, Debug)]
pub enum PrintSeparator {
    Comma,     // Tab to next column
    Semicolon, // No spacing
    None,      // End of PRINT statement
}

#[derive(Clone)]
pub enum Stmt {
    Label(i32),
    Let(String, Expr),
    Print(Vec<PrintItem>),
    Goto(i32),
    IfGoto {
        lhs: Expr,
        op: Cmp,
        rhs: Expr,
        target: i32,
    },
    For {
        var: String,
        start: Expr,
        end: Expr,
        step: Option<Expr>,
    },
    Next(Option<String>),
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

#[derive(Clone, Copy, Debug)]
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
