// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

use basic_dsl::basic;

fn main() {
    basic!(
        // language=basic
        r#"
        10 LET X = 1
        20 PRINT X
        30 IF X < 5 THEN GOTO 50
        35 PRINT "DONE"
        40 END
        50 LET X = X + 1
        60 GOTO 20
        "#
    );
}
