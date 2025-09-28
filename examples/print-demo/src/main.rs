// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

use basic_dsl::basic;

fn main() {
    println!("=== Enhanced PRINT Statement Demo ===");
    basic! {
        10 PRINT "Testing multiple expressions:"
        20 LET X = 42
        30 LET Y = 24
        40 PRINT "X =", X, "Y =", Y
        50 PRINT "Sum:", X + Y
        60 PRINT
        70 PRINT "Empty PRINT above creates a newline"
        80 PRINT "String", "Number", 123, "Variable", X
        90 END
    }
}