use basic_dsl::{basic, basic_compiled};

fn main() {
    println!("=== Control Flow Test ===");
    
    println!("\n--- Interpreted Version (with GOTO) ---");
    basic! {
        10 LET X = 1
        20 PRINT X
        30 LET X = X + 1  
        40 IF X <= 3 THEN GOTO 20
        50 PRINT "Done with loop"
        60 END
    }
    
    println!("\n--- Compiled Version (no GOTO support) ---");
    // This won't compile if GOTO isn't implemented
    // basic_compiled! {
    //     10 LET X = 1
    //     20 PRINT X
    //     30 LET X = X + 1  
    //     40 IF X <= 3 THEN GOTO 20
    //     50 PRINT "Done"
    //     60 END
    // }
    println!("(GOTO not supported in compiled version yet)");
}