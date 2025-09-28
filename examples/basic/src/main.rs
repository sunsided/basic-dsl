// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

use basic_dsl::basic;

fn main() {
    basic! {
        10 PRINT "RustBasic FizzBuzz from 1 to 20:"
        20 FOR I = 1 TO 20
        30 LET A = I / 3
        40 LET B = A * 3
        50 LET C = I / 5
        60 LET D = C * 5
        70 IF B = I THEN GOTO 100
        80 IF D = I THEN GOTO 170
        90 PRINT I
        95 GOTO 180
        100 IF D = I THEN GOTO 150
        110 PRINT "Rust"
        120 GOTO 180
        150 PRINT "RustBasic"
        160 GOTO 180
        170 PRINT "Basic"
        180 NEXT I
        190 PRINT "DONE"
        200 END
    }
}
