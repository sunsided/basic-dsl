// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

/*!
# BASIC DSL

A procedural macro crate that provides a BASIC interpreter DSL embedded in Rust.

## Usage

```rust
use basic_dsl::basic;

basic!(r#"
    10 LET X = 1
    20 PRINT X
    30 IF X < 5 THEN GOTO 50
    40 END
    50 LET X = X + 1
    60 GOTO 20
"#);
```

The macro supports:
- Variable assignment with `LET`
- Printing with `PRINT` (numbers and string literals)
- Conditional jumps with `IF...THEN GOTO`
- Unconditional jumps with `GOTO`
- Line labels and program termination with `END`
*/

mod ast;
mod codegen;
mod parser;

use proc_macro::TokenStream;
use syn::{Result, spanned::Spanned};

use crate::codegen::generate_runtime_code;
use crate::parser::parse_basic_program;

/// Main entry point for the BASIC DSL macro.
///
/// Accepts only string literals containing BASIC program code.
#[proc_macro]
pub fn basic(input: TokenStream) -> TokenStream {
    match expand_from_lit(input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn expand_from_lit(input: TokenStream) -> Result<proc_macro2::TokenStream> {
    let expr: syn::Expr = syn::parse(input)?;

    let lit = match expr {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(lit),
            ..
        }) => lit,
        other => return Err(syn::Error::new(other.span(), "expected a string literal")),
    };

    expand_basic(&lit)
}

fn expand_basic(src: &syn::LitStr) -> Result<proc_macro2::TokenStream> {
    let code = src.value();
    let stmts = parse_basic_program(&code, src)?;
    generate_runtime_code(stmts, src)
}
