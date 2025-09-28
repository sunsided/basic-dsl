// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

//! Token-based parser for the BASIC DSL

use crate::ast::{Bin, Cmp, Expr, PrintItem, PrintSeparator, Stmt};
use syn::{Ident, LitInt, Result, Token, parse::Parse, parse::ParseStream};

/// Parses a complete BASIC program from a token stream
pub fn parse_basic_program_tokens(input: proc_macro2::TokenStream) -> Result<Vec<Stmt>> {
    let lines = split_token_stream_by_lines(input)?;
    let mut statements = Vec::new();

    for line_tokens in lines {
        if line_tokens.is_empty() {
            continue; // Skip empty lines
        }

        let stmt = syn::parse2::<BasicLine>(line_tokens)?;
        statements.extend(stmt.into_statements());
    }

    Ok(statements)
}

/// Represents a single line in a BASIC program
struct BasicLine {
    line_number: Option<i32>,
    statement: Option<BasicStatement>,
}

impl BasicLine {
    fn into_statements(self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        if let Some(line_num) = self.line_number {
            stmts.push(Stmt::Label(line_num));
        }

        if let Some(stmt) = self.statement {
            stmts.push(stmt.into_ast());
        }

        stmts
    }
}

impl Parse for BasicLine {
    fn parse(input: ParseStream) -> Result<Self> {
        // Try to parse optional line number
        let line_number = if input.peek(LitInt) {
            let lit: LitInt = input.parse()?;
            Some(lit.base10_parse()?)
        } else {
            None
        };

        // If there's nothing left after line number, it's just a label
        let statement = if input.is_empty() {
            None
        } else {
            Some(input.parse()?)
        };

        Ok(BasicLine {
            line_number,
            statement,
        })
    }
}

/// Represents a BASIC statement without line number
enum BasicStatement {
    Let {
        var: String,
        expr: BasicExpr,
    },
    Print(Vec<BasicPrintItem>),
    Input {
        prompt: Option<String>,
        var: String,
    },
    Goto(i32),
    IfGoto {
        lhs: BasicExpr,
        op: Cmp,
        rhs: BasicExpr,
        target: i32,
    },
    For {
        var: String,
        start: BasicExpr,
        end: BasicExpr,
        step: Option<BasicExpr>,
    },
    Next(Option<String>),
    End,
}

impl BasicStatement {
    fn into_ast(self) -> Stmt {
        match self {
            BasicStatement::Let { var, expr } => Stmt::Let(var, expr.into_ast()),
            BasicStatement::Print(exprs) => {
                Stmt::Print(exprs.into_iter().map(|e| e.into_ast()).collect())
            }
            BasicStatement::Input { prompt, var } => Stmt::Input { prompt, var },
            BasicStatement::Goto(target) => Stmt::Goto(target),
            BasicStatement::IfGoto {
                lhs,
                op,
                rhs,
                target,
            } => Stmt::IfGoto {
                lhs: lhs.into_ast(),
                op,
                rhs: rhs.into_ast(),
                target,
            },
            BasicStatement::For {
                var,
                start,
                end,
                step,
            } => Stmt::For {
                var,
                start: start.into_ast(),
                end: end.into_ast(),
                step: step.map(|s| s.into_ast()),
            },
            BasicStatement::Next(var) => Stmt::Next(var),
            BasicStatement::End => Stmt::End,
        }
    }
}

impl Parse for BasicStatement {
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword: Ident = input.parse()?;
        let kw_str = keyword.to_string().to_uppercase();

        match kw_str.as_str() {
            "LET" => {
                let var: Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                let expr = input.parse()?;
                Ok(BasicStatement::Let {
                    var: var.to_string(),
                    expr,
                })
            }
            "PRINT" => {
                let mut print_items = Vec::new();

                // Check if there are any expressions after PRINT
                if !input.is_empty() {
                    loop {
                        // Parse expression
                        let expr = parse_basic_expr_no_comparison(input)?;

                        // Determine separator
                        let separator = if input.peek(Token![,]) {
                            input.parse::<Token![,]>()?; // consume comma
                            PrintSeparator::Comma
                        } else if input.peek(Token![;]) {
                            input.parse::<Token![;]>()?; // consume semicolon
                            PrintSeparator::Semicolon
                        } else {
                            PrintSeparator::None
                        };

                        print_items.push(BasicPrintItem { expr, separator });

                        // If no separator or end of input, break
                        if matches!(separator, PrintSeparator::None) || input.is_empty() {
                            break;
                        }
                    }
                }

                Ok(BasicStatement::Print(print_items))
            }
            "INPUT" => {
                // Parse INPUT statement: INPUT "prompt", var or INPUT var
                let mut prompt = None;
                
                // Check if first token is a string (prompt)
                if input.peek(syn::LitStr) {
                    let prompt_lit: syn::LitStr = input.parse()?;
                    prompt = Some(prompt_lit.value());
                    
                    // Expect comma after prompt
                    input.parse::<Token![,]>()?;
                }
                
                // Parse variable name
                let var: Ident = input.parse()?;
                
                Ok(BasicStatement::Input {
                    prompt,
                    var: var.to_string(),
                })
            }
            "GOTO" => {
                let target: LitInt = input.parse()?;
                Ok(BasicStatement::Goto(target.base10_parse()?))
            }
            "IF" => {
                let lhs = parse_basic_expr_no_comparison(input)?;
                let op = parse_comparison_op(input)?;
                let rhs = parse_basic_expr_no_comparison(input)?;

                let then_keyword: Ident = input.parse()?;
                if then_keyword.to_string().to_uppercase() != "THEN" {
                    return Err(syn::Error::new(then_keyword.span(), "expected THEN"));
                }

                let goto_keyword: Ident = input.parse()?;
                if goto_keyword.to_string().to_uppercase() != "GOTO" {
                    return Err(syn::Error::new(goto_keyword.span(), "expected GOTO"));
                }

                let target: LitInt = input.parse()?;
                Ok(BasicStatement::IfGoto {
                    lhs,
                    op,
                    rhs,
                    target: target.base10_parse()?,
                })
            }
            "FOR" => {
                let var: Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                let start = input.parse()?;

                let to_keyword: Ident = input.parse()?;
                if to_keyword.to_string().to_uppercase() != "TO" {
                    return Err(syn::Error::new(to_keyword.span(), "expected TO"));
                }

                let end = input.parse()?;

                let step = if input.peek(Ident) {
                    let step_keyword: Ident = input.parse()?;
                    if step_keyword.to_string().to_uppercase() == "STEP" {
                        Some(input.parse()?)
                    } else {
                        return Err(syn::Error::new(step_keyword.span(), "unexpected token"));
                    }
                } else {
                    None
                };

                Ok(BasicStatement::For {
                    var: var.to_string(),
                    start,
                    end,
                    step,
                })
            }
            "NEXT" => {
                let var = if input.peek(Ident) {
                    let var_ident: Ident = input.parse()?;
                    Some(var_ident.to_string())
                } else {
                    None
                };
                Ok(BasicStatement::Next(var))
            }
            "END" => Ok(BasicStatement::End),
            _ => Err(syn::Error::new(
                keyword.span(),
                format!("unknown statement: {}", kw_str),
            )),
        }
    }
}

/// Wrapper around expressions to handle BASIC-specific parsing
struct BasicExpr(syn::Expr);

/// Represents a PRINT item with its separator
struct BasicPrintItem {
    expr: BasicExpr,
    separator: PrintSeparator,
}

impl BasicPrintItem {
    fn into_ast(self) -> PrintItem {
        PrintItem {
            expr: self.expr.into_ast(),
            separator: self.separator,
        }
    }
}

impl BasicExpr {
    fn into_ast(self) -> Expr {
        convert_syn_expr_to_basic(self.0)
    }
}

impl Parse for BasicExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(BasicExpr(input.parse()?))
    }
}

/// Convert syn::Expr to our BASIC Expr AST
fn convert_syn_expr_to_basic(expr: syn::Expr) -> Expr {
    match expr {
        syn::Expr::Lit(syn::ExprLit { lit, .. }) => match lit {
            syn::Lit::Int(lit_int) => {
                // Parse as i64, defaulting to 0 if parsing fails
                Expr::Num(lit_int.base10_parse().unwrap_or(0))
            }
            syn::Lit::Str(lit_str) => Expr::Str(lit_str.value()),
            _ => Expr::Num(0), // Fallback for other literal types
        },
        syn::Expr::Path(syn::ExprPath { path, .. }) => {
            // Simple identifier
            if let Some(ident) = path.get_ident() {
                Expr::Var(ident.to_string())
            } else {
                Expr::Num(0) // Fallback
            }
        }
        syn::Expr::Binary(syn::ExprBinary {
            left, op, right, ..
        }) => {
            let lhs = Box::new(convert_syn_expr_to_basic(*left));
            let rhs = Box::new(convert_syn_expr_to_basic(*right));
            let bin_op = match op {
                syn::BinOp::Add(_) => Bin::Add,
                syn::BinOp::Sub(_) => Bin::Sub,
                syn::BinOp::Mul(_) => Bin::Mul,
                syn::BinOp::Div(_) => Bin::Div,
                _ => Bin::Add, // Fallback
            };
            Expr::Bin {
                lhs,
                op: bin_op,
                rhs,
            }
        }
        syn::Expr::Paren(syn::ExprParen { expr, .. }) => convert_syn_expr_to_basic(*expr),
        _ => Expr::Num(0), // Fallback for unsupported expressions
    }
}

/// Parse a BASIC expression that doesn't include comparison operators
fn parse_basic_expr_no_comparison(input: ParseStream) -> Result<BasicExpr> {
    // We'll parse basic expressions manually to avoid consuming comparison operators
    parse_basic_additive(input)
}

/// Parse additive expressions (+ and -)
fn parse_basic_additive(input: ParseStream) -> Result<BasicExpr> {
    let mut left = parse_basic_multiplicative(input)?;

    while input.peek(Token![+]) || input.peek(Token![-]) {
        let op_token = if input.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            syn::BinOp::Add(syn::token::Plus::default())
        } else {
            input.parse::<Token![-]>()?;
            syn::BinOp::Sub(syn::token::Minus::default())
        };

        let right = parse_basic_multiplicative(input)?;
        left = BasicExpr(syn::Expr::Binary(syn::ExprBinary {
            attrs: vec![],
            left: Box::new(left.0),
            op: op_token,
            right: Box::new(right.0),
        }));
    }

    Ok(left)
}

/// Parse multiplicative expressions (* and /)
fn parse_basic_multiplicative(input: ParseStream) -> Result<BasicExpr> {
    let mut left = parse_basic_primary(input)?;

    while input.peek(Token![*]) || input.peek(Token![/]) {
        let op_token = if input.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            syn::BinOp::Mul(syn::token::Star::default())
        } else {
            input.parse::<Token![/]>()?;
            syn::BinOp::Div(syn::token::Slash::default())
        };

        let right = parse_basic_primary(input)?;
        left = BasicExpr(syn::Expr::Binary(syn::ExprBinary {
            attrs: vec![],
            left: Box::new(left.0),
            op: op_token,
            right: Box::new(right.0),
        }));
    }

    Ok(left)
}

/// Parse primary expressions (numbers, strings, variables, parentheses)
fn parse_basic_primary(input: ParseStream) -> Result<BasicExpr> {
    if input.peek(LitInt) {
        let lit: LitInt = input.parse()?;
        Ok(BasicExpr(syn::Expr::Lit(syn::ExprLit {
            attrs: vec![],
            lit: syn::Lit::Int(lit),
        })))
    } else if input.peek(syn::LitStr) {
        let lit: syn::LitStr = input.parse()?;
        Ok(BasicExpr(syn::Expr::Lit(syn::ExprLit {
            attrs: vec![],
            lit: syn::Lit::Str(lit),
        })))
    } else if input.peek(syn::token::Paren) {
        let content;
        syn::parenthesized!(content in input);
        let inner = parse_basic_expr_no_comparison(&content)?;
        Ok(BasicExpr(syn::Expr::Paren(syn::ExprParen {
            attrs: vec![],
            paren_token: syn::token::Paren::default(),
            expr: Box::new(inner.0),
        })))
    } else if input.peek(Ident) {
        let ident: Ident = input.parse()?;
        Ok(BasicExpr(syn::Expr::Path(syn::ExprPath {
            attrs: vec![],
            qself: None,
            path: syn::Path::from(ident),
        })))
    } else {
        Err(input.error("expected number, string, variable, or parentheses"))
    }
}

/// Parse comparison operators
fn parse_comparison_op(input: ParseStream) -> Result<Cmp> {
    if input.peek(Token![<]) {
        input.parse::<Token![<]>()?;
        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Ok(Cmp::Le)
        } else {
            Ok(Cmp::Lt)
        }
    } else if input.peek(Token![>]) {
        input.parse::<Token![>]>()?;
        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Ok(Cmp::Ge)
        } else {
            Ok(Cmp::Gt)
        }
    } else if input.peek(Token![=]) {
        input.parse::<Token![=]>()?;
        Ok(Cmp::Eq)
    } else {
        Err(input.error("expected comparison operator (<, <=, =, >=, >)"))
    }
}

/// Split a token stream into lines based on line boundaries
fn split_token_stream_by_lines(
    input: proc_macro2::TokenStream,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut lines = Vec::new();
    let mut current_line = Vec::new();
    let mut current_line_num: Option<usize> = None;

    for token in input.into_iter() {
        let token_line = token.span().start().line;

        if let Some(line_num) = current_line_num
            && token_line != line_num {
                // We've moved to a new line, finish current line
                if !current_line.is_empty() {
                    lines.push(current_line.clone().into_iter().collect());
                    current_line.clear();
                }
            }

        current_line_num = Some(token_line);
        current_line.push(token);
    }

    // Don't forget the last line
    if !current_line.is_empty() {
        lines.push(current_line.into_iter().collect());
    }

    Ok(lines)
}
