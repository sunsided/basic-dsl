// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

//! Parser for the BASIC DSL

use syn::Result;
use crate::ast::{Stmt, Expr, Bin, Cmp};

pub fn parse_basic_program(code: &str, src: &syn::LitStr) -> Result<Vec<Stmt>> {
    let mut stmts = Vec::<Stmt>::new();

    for raw in code.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        let (maybe_label, rest) = if let Some((n, r)) = split_leading_number(line) {
            (Some(n), r)
        } else {
            (None, line)
        };

        if let Some(n) = maybe_label {
            stmts.push(Stmt::Label(n));
        }

        let mut w = split_ws_preserving_ops(rest);
        if w.is_empty() {
            continue;
        }

        let kw = up(&w[0]);

        if kw == "LET" {
            w.remove(0);
            let var = w.remove(0);
            expect(&mut w, "=")?;
            let e = parse_expr(&mut w)?;
            expect_eol(w)?;
            stmts.push(Stmt::Let(var, e));
            continue;
        }

        if kw == "PRINT" {
            w.remove(0);
            let e = parse_expr(&mut w)?;
            expect_eol(w)?;
            stmts.push(Stmt::Print(e));
            continue;
        }

        if kw == "GOTO" {
            w.remove(0);
            let tgt: i32 = w.remove(0).parse()
                .map_err(|_| err(src, "GOTO needs a number"))?;
            expect_eol(w)?;
            stmts.push(Stmt::Goto(tgt));
            continue;
        }

        if kw == "IF" {
            w.remove(0);
            let lhs = parse_expr(&mut w)?;
            let op = parse_cmp(&mut w)?;
            let rhs = parse_expr(&mut w)?;

            if up(&w.remove(0)) != "THEN" {
                return Err(err(src, "expected THEN"));
            }
            if up(&w.remove(0)) != "GOTO" {
                return Err(err(src, "expected GOTO"));
            }

            let tgt: i32 = w.remove(0).parse()
                .map_err(|_| err(src, "IF..THEN GOTO needs a number"))?;
            expect_eol(w)?;
            stmts.push(Stmt::IfGoto { lhs, op, rhs, target: tgt });
            continue;
        }

        if kw == "END" {
            expect_eol(w.split_off(1))?;
            stmts.push(Stmt::End);
            continue;
        }

        return Err(err(src, &format!("unknown statement: {line}")));
    }

    Ok(stmts)
}

fn up(s: &str) -> String {
    s.to_ascii_uppercase()
}

fn err(src: &syn::LitStr, msg: &str) -> syn::Error {
    syn::Error::new(src.span(), msg)
}

fn split_leading_number(s: &str) -> Option<(i32, &str)> {
    let mut it = s.trim_start().splitn(2, char::is_whitespace);
    let first = it.next()?;

    if let Ok(n) = first.parse::<i32>() {
        Some((n, it.next().unwrap_or("")))
    } else {
        None
    }
}

fn split_ws_preserving_ops(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;

    let flush = |cur: &mut String, out: &mut Vec<String>| {
        if !cur.is_empty() {
            out.push(std::mem::take(cur));
        }
    };

    for ch in s.chars() {
        if ch == '"' {
            cur.push(ch);
            if in_quotes {
                // End of quoted string
                flush(&mut cur, &mut out);
                in_quotes = false;
            } else {
                // Start of quoted string
                in_quotes = true;
            }
        } else if in_quotes {
            // Inside quotes, add everything including whitespace
            cur.push(ch);
        } else if ch.is_whitespace() {
            flush(&mut cur, &mut out);
        } else if "=+-*/()<>".contains(ch) {
            flush(&mut cur, &mut out);
            out.push(ch.to_string());
        } else {
            cur.push(ch);
        }
    }

    flush(&mut cur, &mut out);
    out
}

fn expect(words: &mut Vec<String>, token: &str) -> Result<()> {
    if words.first().map(|s| s.as_str()) == Some(token) {
        words.remove(0);
        Ok(())
    } else {
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("expected `{token}`")
        ))
    }
}

fn expect_eol(words: Vec<String>) -> Result<()> {
    if words.is_empty() {
        Ok(())
    } else {
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "trailing tokens"
        ))
    }
}

fn parse_cmp(words: &mut Vec<String>) -> Result<Cmp> {
    let t = words.remove(0);
    Ok(match t.as_str() {
        "<" => Cmp::Lt,
        "<=" => Cmp::Le,
        "=" => Cmp::Eq,
        ">=" => Cmp::Ge,
        ">" => Cmp::Gt,
        _ => return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected comparison op"
        )),
    })
}

fn parse_expr(words: &mut Vec<String>) -> Result<Expr> {
    parse_expr_bp(words, 0)
}

fn parse_expr_bp(words: &mut Vec<String>, min_bp: u8) -> Result<Expr> {
    let mut lhs = parse_atom(words)?;

    while let Some((op, lbp, rbp)) = peek_bin(words) {
        if lbp < min_bp {
            break;
        }

        words.remove(0);
        let rhs = parse_expr_bp(words, rbp)?;
        lhs = Expr::Bin {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs)
        };
    }

    Ok(lhs)
}

fn parse_atom(words: &mut Vec<String>) -> Result<Expr> {
    let t = words.remove(0);

    if let Ok(n) = t.parse::<i64>() {
        return Ok(Expr::Num(n));
    }

    if t.starts_with('"') && t.ends_with('"') && t.len() >= 2 {
        // Parse string literal, removing quotes
        let content = &t[1..t.len()-1];
        return Ok(Expr::Str(content.to_string()));
    }

    if t.chars().all(|c| c == '_' || c.is_ascii_alphanumeric()) {
        return Ok(Expr::Var(t));
    }

    Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "expected number, string literal, or identifier"
    ))
}

fn peek_bin(words: &[String]) -> Option<(Bin, u8, u8)> {
    use Bin::*;
    match words.first().map(|s| s.as_str())? {
        "+" => Some((Add, 1, 2)),
        "-" => Some((Sub, 1, 2)),
        "*" => Some((Mul, 3, 4)),
        "/" => Some((Div, 3, 4)),
        _ => None,
    }
}
