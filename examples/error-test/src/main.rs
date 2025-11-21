use basic_dsl::{basic, basic_compiled};

fn main() {
    println!("=== Testing Error Handling ===");
    
    // This should cause a divide by zero
    println!("\n--- Interpreted Version ---");
    basic! {
        10 LET X = 0
        20 LET Y = 5 / X  // Division by zero
        30 PRINT Y
        40 END
    }
    
    println!("\n--- Compiled Version ---");
    basic_compiled! {
        10 LET X = 0  
        20 LET Y = 5 / X  // Division by zero
        30 PRINT Y
        40 END
    }
}