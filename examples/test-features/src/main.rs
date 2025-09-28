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
        40 PRINT "X =", X
        50 PRINT "Y =", Y
        60 PRINT "Z = X * Y + 2 =", Z
        70 PRINT
        80 IF Z > 15 THEN GOTO 100
        90 PRINT "Z is not greater than 15"
        100 PRINT "Program complete"
        110 END
    }
}
