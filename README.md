# BASIC DSL for Rust 🦀

A Rust procedural macro crate that provides a BASIC interpreter embedded as a domain-specific language with direct token parsing for excellent IDE support.

## Features

- **Variables**: Assignment with `LET` and arithmetic expressions
- **Input/Output**: `INPUT` for reading user input and `PRINT` statements with flexible formatting
- **Control Flow**: `IF...THEN GOTO` conditionals and `GOTO` jumps
- **Loops**: `FOR...NEXT` loops with optional `STEP` increment
- **Program Structure**: Line numbers, labels, and `END` termination
- **IDE Support**: Full syntax highlighting, error reporting, and code completion

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
basic-dsl = "0.2.0"
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
- `PRINT [expression, ...]` - Output values or strings (comma creates tab stops for columnar formatting, semicolon concatenates, or empty for newline)
- `INPUT [prompt,] variable` - Read user input from stdin into variable (auto-detects numbers vs strings)
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

### Enhanced PRINT Statement
```rust
basic! {
    10 PRINT "Name", "Age", "Score"    // Tab-separated columns
    20 PRINT "Alice", 25, 95           // Comma creates tab stops  
    30 PRINT "Bob", 30, 87             // For columnar output
    40 PRINT "Concat:"; "A"; "B"       // Semicolon concatenates
    50 PRINT                           // Empty PRINT = newline
    60 END
}
```

### Interactive INPUT
```rust
basic! {
    10 PRINT "Number Guessing Game"
    20 INPUT "Enter your guess", GUESS
    30 IF GUESS = 42 THEN GOTO 60
    40 PRINT "Wrong! The answer was 42"
    50 GOTO 70
    60 PRINT "Correct! You got it!"
    70 END
}
```

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
# Run the RustBasic FitzBuzz example
cargo run --bin basic

# Run comprehensive feature tests  
cargo run --bin test-features

# Run enhanced PRINT statement demo
cargo run --bin print-demo

# Test INPUT functionality with various scenarios
cargo run --bin input-test

# Play the interactive number guessing game
cargo run --bin number-guessing-game
```

## License

Licensed under the European Union Public Licence 1.2 (EUPL-1.2).
