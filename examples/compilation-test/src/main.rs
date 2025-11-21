use basic_dsl::basic_compiled;

fn main() {
    println!("=== Testing Direct Compilation ===");
    
    basic_compiled! {
        10 LET X = 42
        20 PRINT X
        30 LET Y = X + 8  
        40 PRINT Y
        50 END
    }
    
    println!("=== Compilation Test Complete ===");
}