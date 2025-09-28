use basic_dsl::basic_compiled;

fn main() {
    println!("=== Testing Compiled INPUT ===");
    
    basic_compiled! {
        10 INPUT "Enter a number", X
        20 LET Y = X * 2
        30 PRINT Y
        40 END
    }
}