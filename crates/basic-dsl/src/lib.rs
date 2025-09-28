// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

/*!
# BASIC DSL

A procedural macro crate that provides a BASIC interpreter DSL embedded in Rust.

## Usage

```rust
use basic_dsl::basic;

basic! {
    10 FOR I = 1 TO 5
    20 PRINT I
    30 NEXT I
    40 PRINT "DONE"
    50 END
}
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
mod token_parser;

use proc_macro::TokenStream;
use syn::Result;

use crate::codegen::generate_runtime_code;
use crate::token_parser::parse_basic_program_tokens;

/// Main entry point for the BASIC DSL macro.
///
/// Accepts BASIC program code as direct tokens.
#[proc_macro]
pub fn basic(input: TokenStream) -> TokenStream {
    match expand_from_tokens(input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn expand_from_tokens(input: TokenStream) -> Result<proc_macro2::TokenStream> {
    let tokens = proc_macro2::TokenStream::from(input);
    let stmts = parse_basic_program_tokens(tokens)?;
    generate_runtime_code(stmts, &syn::LitStr::new("", proc_macro2::Span::call_site()))
}
