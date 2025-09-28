# BASIC DSL for Rust 🦀

A Rust procedural macro crate that provides a BASIC interpreter embedded as a domain-specific language with direct token parsing for excellent IDE support.

## Features

- **Variables**: Assignment with `LET` and arithmetic expressions
- **Output**: `PRINT` statements for numbers and string literals  
- **Control Flow**: `IF...THEN GOTO` conditionals and `GOTO` jumps
- **Loops**: `FOR...NEXT` loops with optional `STEP` increment
- **Program Structure**: Line numbers, labels, and `END` termination
- **IDE Support**: Full syntax highlighting, error reporting, and code completion

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
basic-dsl = "0.1.1"
```

Then use the macro in your Rust code:

```rust
use basic_dsl::basic;

fn main() {
    basic! {
        10 LET X = 1
        20 PRINT X
        30 IF X < 5 THEN GOTO 50
        40 PRINT "DONE"
        45 END
        50 LET X = X + 1
        60 GOTO 20
    }
}
```

## Syntax

The DSL supports classic BASIC syntax with modern IDE integration:

### Statements
- `LET variable = expression` - Variable assignment
- `PRINT expression` - Output values or strings
- `GOTO label` - Jump to line number
- `IF condition THEN GOTO label` - Conditional jump  
- `FOR variable = start TO end [STEP increment]` - Loop initialization
- `NEXT [variable]` - Loop increment and condition check
- `END` - Terminate program

### Expressions
- **Arithmetic**: `+`, `-`, `*`, `/` with proper precedence
- **Comparisons**: `<`, `<=`, `=`, `>=`, `>` 
- **Literals**: Numbers (`42`) and strings (`"Hello"`)
- **Variables**: Any valid identifier (`X`, `COUNTER`, etc.)
- **Parentheses**: For grouping expressions

### Program Structure
- Line numbers act as labels for jumps and program flow
- Statements are written directly without quotes for full IDE support
- Nested FOR loops and complex expressions are fully supported

## Examples

### RustBasic FizzBuzz
```rust
basic! {
    10 PRINT "RustBasic FizzBuzz from 1 to 20:"
    20 FOR I = 1 TO 20
    30 LET A = I / 3 * 3
    40 LET B = I / 5 * 5
    50 IF A = I THEN GOTO 100
    60 IF B = I THEN GOTO 170
    70 PRINT I
    80 GOTO 180
    100 IF B = I THEN GOTO 150
    110 PRINT "Rust"
    120 GOTO 180
    150 PRINT "RustBasic"
    160 GOTO 180
    170 PRINT "Basic"
    180 NEXT I
    190 END
}
```

### Running Examples
This repository includes example programs that you can run:

```bash
# Run the RustBasic FizzBuzz example
cargo run --bin basic

# Run comprehensive feature tests  
cargo run --bin test-features
```

## License

Licensed under the European Union Public Licence 1.2 (EUPL-1.2).
