// SPDX-License-Identifier: EUPL-1.2
// Copyright (c) 2025 The BASIC DSL Contributors

#[test]
fn compile() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile/pass_inline.rs");
    t.compile_fail("tests/compile/fail_unknown_stmt.rs");
}
