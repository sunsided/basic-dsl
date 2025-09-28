// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

use basic_dsl::basic;

fn main() {
    println!("=== Enhanced PRINT Statement Demo ===");
    basic! {
        10 PRINT "BASIC-Style Comma Formatting:"
        20 PRINT "Name", "Age", "Score", "Grade"
        30 PRINT "Alice", 25, 95, "A"
        40 PRINT "Bob", 30, 87, "B"
        50 PRINT "Charlie", 28, 92, "A"
        60 PRINT
        70 PRINT "Variables work too:"
        80 LET X = 42
        90 LET Y = 24
        100 PRINT "X =", X, "Y =", Y
        110 PRINT "Sum:", X + Y
        120 PRINT
        130 PRINT "Semicolon Concatenation:"
        140 PRINT "Hello"; "World"; "!"
        150 PRINT "Numbers:"; 1; 2; 3; 4; 5
        160 PRINT
        170 PRINT "Mixed Separators:"
        180 PRINT "A"; "B", "C"; "D", "E"
        190 END
    }
}
