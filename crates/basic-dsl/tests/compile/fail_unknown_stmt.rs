// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

use basic_dsl::basic;

fn main() {
    basic!(r#"
10 WROTE X = 1   // <- invalid keyword
20 END
"#);
}
