// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

use basic_dsl::basic;

fn main() {
    println!("=== FOR...NEXT Loop Demo ===");
    basic!(
        // language=basic
        r#"
        10 PRINT "Count from 1 to 5:"
        20 FOR I = 1 TO 5
        30 PRINT I
        40 NEXT I
        50 PRINT "Count by 2s from 0 to 10:"
        60 FOR J = 0 TO 10 STEP 2
        70 PRINT J
        80 NEXT J
        90 PRINT "Nested loops - multiplication table:"
        100 FOR A = 1 TO 3
        110 FOR B = 1 TO 3
        120 LET C = A * B
        130 PRINT C
        140 NEXT B
        150 NEXT A
        160 PRINT "DONE"
        170 END
        "#
    );
}
