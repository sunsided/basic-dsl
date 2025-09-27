# BASIC DSL for Rust 🦀

A Rust procedural macro crate that provides a BASIC interpreter embedded as a domain-specific language.

## Features

- Variable assignment with `LET`
- Output with `PRINT` (supports numbers and string literals)
- Conditional jumps with `IF...THEN GOTO`
- Unconditional jumps with `GOTO`
- Line labels and program termination with `END`

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
basic-dsl = "0.1.0"
```

Then use the macro in your Rust code:

```rust
use basic_dsl::basic;

fn main() {
    basic!(r#"
        10 LET X = 1
        20 PRINT X
        30 IF X < 5 THEN GOTO 50
        40 PRINT "DONE"
        45 END
        50 LET X = X + 1
        60 GOTO 20
    "#);
}
```

## Syntax

The DSL supports a subset of BASIC syntax:

- `LET variable = expression` - Variable assignment
- `PRINT expression` - Output values or strings
- `GOTO label` - Jump to line number
- `IF condition THEN GOTO label` - Conditional jump
- `END` - Terminate program
- Line numbers act as labels for jumps

Expressions support basic arithmetic operations (`+`, `-`, `*`, `/`) and string literals in double quotes.

## License

Licensed under the European Union Public Licence 1.2 (EUPL-1.2).
