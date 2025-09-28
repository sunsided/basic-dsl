// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

use basic_dsl::basic;

fn main() {
    println!("=== Testing All BASIC Features ===");
    
    // Test IF...THEN GOTO and expressions
    basic! {
        10 LET X = 5
        20 LET Y = 3
        30 LET Z = X * Y + 2
        40 PRINT "X ="
        50 PRINT X
        60 PRINT "Y ="  
        70 PRINT Y
        80 PRINT "Z = X * Y + 2 ="
        90 PRINT Z
        100 IF Z > 15 THEN GOTO 120
        110 PRINT "Z is not greater than 15"
        120 PRINT "Program complete"
        130 END
    }
}
