// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

/*!
# BASIC DSL

A procedural macro crate that provides a BASIC interpreter DSL embedded in Rust.

## Usage

```rust
use basic_dsl::basic;

basic!(r#"
    10 FOR I = 1 TO 5
    20 PRINT I
    30 NEXT I
    40 PRINT "DONE"
    50 END
"#);
```

The macro supports classic BASIC programming constructs:

### Control Flow
- **FOR...NEXT loops**: `FOR variable = start TO end [STEP increment]` and `NEXT [variable]`
- **Conditional jumps**: `IF condition THEN GOTO line`
- **Unconditional jumps**: `GOTO line`
- **Program termination**: `END`

### Variables and Expressions
- **Variable assignment**: `LET variable = expression`
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Comparisons**: `<`, `<=`, `=`, `>=`, `>`
- **Numbers and string literals**: `42`, `"Hello World"`

### Input/Output
- **Printing**: `PRINT expression` (numbers and strings)

### Advanced Features
- **Nested loops**: Full support for nested FOR...NEXT constructs
- **Expression evaluation**: Complex arithmetic and string handling
- **Runtime error checking**: Proper error handling for invalid operations
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
