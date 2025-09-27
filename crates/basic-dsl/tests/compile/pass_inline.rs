// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

use basic_dsl::basic;

fn main() {
    basic!(r#"
10 LET X = 1
20 PRINT X
30 IF X < 3 THEN GOTO 50
40 END
50 LET X = X + 1
60 GOTO 20
"#);
}
