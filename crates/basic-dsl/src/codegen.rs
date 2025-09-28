// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

//! Code generation for the BASIC DSL

use crate::ast::{Atom, Bin, Cmp, Expr, Stmt};
use quote::quote;
use std::collections::BTreeMap;
use syn::Result;

pub fn generate_runtime_code(
    stmts: Vec<Stmt>,
    src: &syn::LitStr,
) -> Result<proc_macro2::TokenStream> {
    let labels = build_label_map(&stmts);
    let mut vars = BTreeMap::<String, usize>::new();
    let mut pool: Vec<Vec<Atom>> = Vec::new();
    let code_ts = generate_bytecode(&stmts, &labels, &mut pool, &mut vars, src)?;
    let pool_ts = generate_expression_pool(&pool, src);
    let var_name_lits = generate_variable_names(&vars, src);

    Ok(quote!({
        #[allow(non_camel_case_types, unused_variables, unused_mut)]
        enum Bin { Add, Sub, Mul, Div }

        #[allow(non_camel_case_types)]
        enum Cmp { Lt, Le, Eq, Ge, Gt }

        #[derive(Clone, Debug)]
        enum Value { Num(i64), Str(String) }

        enum Atom { Imm(i64), Str(&'static str), Var(usize), Bin(Bin) }

        enum Op { End, Print(&'static [usize]), Let(usize,usize), Goto(usize), IfCmp(usize,usize,Cmp,usize), For(usize,usize,usize,usize), Next(Option<usize>) }

        #[derive(Clone)]
        struct LoopState { var: usize, end: usize, step: usize, start_pc: usize }

        static EXPRS: &[&[Atom]] = &[ #(#pool_ts),* ];
        static CODE: &[Op] = &[ #(#code_ts),* ];
        static VARS: &[&str] = &[ #(#var_name_lits),* ];

        let mut vars = vec![Value::Num(0); VARS.len()];
        let mut loop_stack: Vec<LoopState> = Vec::new();

        fn eval(ix: usize, vars: &mut [Value]) -> Value {
            let mut st: Vec<Value> = Vec::new();

            for a in EXPRS[ix] {
                match a {
                    Atom::Imm(n) => st.push(Value::Num(*n)),
                    Atom::Str(s) => st.push(Value::Str(s.to_string())),
                    Atom::Var(i) => st.push(vars[*i].clone()),
                    Atom::Bin(Bin::Add) => {
                        let b = st.pop().unwrap();
                        let a = st.pop().unwrap();
                        match (a, b) {
                            (Value::Num(x), Value::Num(y)) => st.push(Value::Num(x + y)),
                            _ => panic!("Cannot add non-numbers"),
                        }
                    }
                    Atom::Bin(Bin::Sub) => {
                        let b = st.pop().unwrap();
                        let a = st.pop().unwrap();
                        match (a, b) {
                            (Value::Num(x), Value::Num(y)) => st.push(Value::Num(x - y)),
                            _ => panic!("Cannot subtract non-numbers"),
                        }
                    }
                    Atom::Bin(Bin::Mul) => {
                        let b = st.pop().unwrap();
                        let a = st.pop().unwrap();
                        match (a, b) {
                            (Value::Num(x), Value::Num(y)) => st.push(Value::Num(x * y)),
                            _ => panic!("Cannot multiply non-numbers"),
                        }
                    }
                    Atom::Bin(Bin::Div) => {
                        let b = st.pop().unwrap();
                        let a = st.pop().unwrap();
                        match (a, b) {
                            (Value::Num(x), Value::Num(y)) => st.push(Value::Num(x / y)),
                            _ => panic!("Cannot divide non-numbers"),
                        }
                    }
                }
            }

            st.pop().unwrap_or(Value::Num(0))
        }

        let mut pc: usize = 0;
        while pc < CODE.len() {
            match CODE[pc] {
                Op::End => break,
                Op::Print(exprs) => {
                    if exprs.is_empty() {
                        // Empty PRINT statement - just print a newline
                        println!();
                    } else {
                        // Print all expressions separated by spaces
                        let values: Vec<String> = exprs.iter().map(|&e| {
                            let v = eval(e, &mut vars);
                            match v {
                                Value::Num(n) => n.to_string(),
                                Value::Str(s) => s,
                            }
                        }).collect();
                        println!("{}", values.join(" "));
                    }
                    pc += 1;
                }
                Op::Let(v, e) => {
                    let x = eval(e, &mut vars);
                    vars[v] = x;
                    pc += 1;
                }
                Op::Goto(to) => {
                    pc = to;
                }
                Op::IfCmp(l, r, ref c, to) => {
                    let a = eval(l, &mut vars);
                    let b = eval(r, &mut vars);
                    let cond = match (a, b) {
                        (Value::Num(x), Value::Num(y)) => match c {
                            Cmp::Lt => x < y,
                            Cmp::Le => x <= y,
                            Cmp::Eq => x == y,
                            Cmp::Ge => x >= y,
                            Cmp::Gt => x > y
                        },
                        _ => panic!("Cannot compare non-numbers"),
                    };
                    pc = if cond { to } else { pc + 1 };
                }
                Op::For(var, start_e, end_e, step_e) => {
                    let start_val = eval(start_e, &mut vars);
                    let end_val = eval(end_e, &mut vars);
                    let step_val = eval(step_e, &mut vars);
                    
                    let (start_num, end_num, step_num) = match (start_val, end_val, step_val) {
                        (Value::Num(s), Value::Num(e), Value::Num(st)) => (s, e, st),
                        _ => panic!("FOR loop bounds must be numbers"),
                    };
                    
                    vars[var] = Value::Num(start_num);
                    loop_stack.push(LoopState {
                        var: var,
                        end: end_num as usize,
                        step: step_num as usize, 
                        start_pc: pc + 1,
                    });
                    pc += 1;
                }
                Op::Next(expected_var) => {
                    if let Some(loop_state) = loop_stack.last() {
                        // Verify variable name if provided
                        if let Some(exp_var) = expected_var {
                            if loop_state.var != exp_var {
                                panic!("NEXT variable mismatch");
                            }
                        }
                        
                        // Increment loop variable
                        let current_val = match &vars[loop_state.var] {
                            Value::Num(n) => *n,
                            _ => panic!("Loop variable must be numeric"),
                        };
                        
                        let new_val = current_val + loop_state.step as i64;
                        vars[loop_state.var] = Value::Num(new_val);
                        
                        // Check if loop should continue
                        if new_val <= loop_state.end as i64 {
                            pc = loop_state.start_pc; // Jump back to loop body
                        } else {
                            loop_stack.pop(); // Exit loop
                            pc += 1;
                        }
                    } else {
                        panic!("NEXT without FOR");
                    }
                }
            }
        }
    }))
}

fn build_label_map(stmts: &[Stmt]) -> BTreeMap<i32, usize> {
    let mut labels = BTreeMap::<i32, usize>::new();
    let mut pc = 0usize;

    for s in stmts {
        match s {
            Stmt::Label(n) => {
                labels.insert(*n, pc);
            }
            _ => pc += 1,
        }
    }

    labels
}

fn generate_bytecode(
    stmts: &[Stmt],
    labels: &BTreeMap<i32, usize>,
    pool: &mut Vec<Vec<Atom>>,
    vars: &mut BTreeMap<String, usize>,
    src: &syn::LitStr,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut code_ts = Vec::<proc_macro2::TokenStream>::new();

    for s in stmts {
        match s {
            Stmt::Label(_) => {}
            Stmt::End => code_ts.push(quote!(Op::End)),
            Stmt::Print(exprs) => {
                let expr_indices: Vec<usize> = exprs.iter()
                    .map(|e| encode_expr(e, pool, vars))
                    .collect();
                code_ts.push(quote!( Op::Print(&[#(#expr_indices),*]) ));
            }
            Stmt::Let(v, e) => {
                let vi = intern_var(vars, v);
                let x = encode_expr(e, pool, vars);
                code_ts.push(quote!( Op::Let(#vi, #x) ));
            }
            Stmt::Goto(t) => {
                let to = *labels
                    .get(t)
                    .ok_or_else(|| syn::Error::new(src.span(), format!("unknown label {t}")))?;
                code_ts.push(quote!( Op::Goto(#to) ));
            }
            Stmt::IfGoto {
                lhs,
                op,
                rhs,
                target,
            } => {
                let l = encode_expr(lhs, pool, vars);
                let r = encode_expr(rhs, pool, vars);
                let to = *labels.get(target).ok_or_else(|| {
                    syn::Error::new(src.span(), format!("unknown label {target}"))
                })?;
                let cmp_ident = syn::Ident::new(
                    match op {
                        Cmp::Lt => "Lt",
                        Cmp::Le => "Le",
                        Cmp::Eq => "Eq",
                        Cmp::Ge => "Ge",
                        Cmp::Gt => "Gt",
                    },
                    src.span(),
                );
                code_ts.push(quote!( Op::IfCmp(#l, #r, Cmp::#cmp_ident, #to) ));
            }
            Stmt::For { var, start, end, step } => {
                let vi = intern_var(vars, var);
                let start_e = encode_expr(start, pool, vars);
                let end_e = encode_expr(end, pool, vars);
                let step_e = if let Some(s) = step {
                    encode_expr(s, pool, vars)
                } else {
                    // Default step of 1
                    let step_atoms = vec![Atom::Imm(1)];
                    pool.push(step_atoms);
                    pool.len() - 1
                };
                code_ts.push(quote!( Op::For(#vi, #start_e, #end_e, #step_e) ));
            }
            Stmt::Next(var_name) => {
                if let Some(v) = var_name {
                    let var_idx = intern_var(vars, v);
                    code_ts.push(quote!( Op::Next(Some(#var_idx)) ));
                } else {
                    code_ts.push(quote!( Op::Next(None) ));
                }
            }
        }
    }

    Ok(code_ts)
}

fn generate_expression_pool(
    pool: &[Vec<Atom>],
    src: &syn::LitStr,
) -> Vec<proc_macro2::TokenStream> {
    let mut pool_ts = Vec::<proc_macro2::TokenStream>::new();

    for e in pool {
        let atoms = e.iter().map(|a| match a {
            Atom::Imm(n) => quote!( Atom::Imm(#n) ),
            Atom::Str(s) => quote!( Atom::Str(#s) ),
            Atom::Var(i) => quote!( Atom::Var(#i) ),
            Atom::Bin(b) => {
                let id = syn::Ident::new(
                    match b {
                        Bin::Add => "Add",
                        Bin::Sub => "Sub",
                        Bin::Mul => "Mul",
                        Bin::Div => "Div",
                    },
                    src.span(),
                );
                quote!( Atom::Bin(Bin::#id) )
            }
        });
        pool_ts.push(quote!( &[#(#atoms),*] ));
    }

    pool_ts
}

fn generate_variable_names(
    vars: &BTreeMap<String, usize>,
    src: &syn::LitStr,
) -> Vec<proc_macro2::TokenStream> {
    let mut by_ix: Vec<Option<String>> = vec![
        None;
        {
            let mut max = 0usize;
            for ix in vars.values() {
                if *ix > max {
                    max = *ix;
                }
            }
            if vars.is_empty() { 0 } else { max + 1 }
        }
    ];

    for (name, ix) in vars {
        if *ix >= by_ix.len() {
            by_ix.resize(*ix + 1, None);
        }
        by_ix[*ix] = Some(name.clone());
    }

    by_ix
        .iter()
        .map(|opt| {
            let s = opt.as_deref().unwrap_or("");
            let lit = syn::LitStr::new(s, src.span());
            quote!( #lit )
        })
        .collect()
}

fn intern_var(vars: &mut BTreeMap<String, usize>, v: &str) -> usize {
    if let Some(&i) = vars.get(v) {
        i
    } else {
        let i = vars.len();
        vars.insert(v.to_string(), i);
        i
    }
}

fn encode_expr(e: &Expr, pool: &mut Vec<Vec<Atom>>, vars: &mut BTreeMap<String, usize>) -> usize {
    fn walk(out: &mut Vec<Atom>, e: &Expr, vars: &mut BTreeMap<String, usize>) {
        match e {
            Expr::Num(n) => out.push(Atom::Imm(*n)),
            Expr::Str(s) => out.push(Atom::Str(s.clone())),
            Expr::Var(s) => {
                let ix = intern_var(vars, s);
                out.push(Atom::Var(ix));
            }
            Expr::Bin { lhs, op, rhs } => {
                walk(out, lhs, vars);
                walk(out, rhs, vars);
                out.push(Atom::Bin(*op));
            }
        }
    }

    let mut v = Vec::new();
    walk(&mut v, e, vars);
    let ix = pool.len();
    pool.push(v);
    ix
}
