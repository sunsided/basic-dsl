use basic_dsl::{basic, basic_compiled};

fn main() {
    println!("=== Comparison: Interpreter vs Compiled ===");
    
    println!("\n--- Interpreter Version ---");
    basic! {
        10 LET X = 42
        20 PRINT X
        30 LET Y = X + 8
        40 PRINT Y
        50 END
    }
    
    println!("\n--- Compiled Version ---");
    basic_compiled! {
        10 LET X = 42
        20 PRINT X
        30 LET Y = X + 8
        40 PRINT Y
        50 END
    }
    
    println!("\n=== Both produce the same result! ===");
}