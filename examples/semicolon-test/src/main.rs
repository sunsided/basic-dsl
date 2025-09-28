use basic_dsl::basic;

fn main() {
    basic! {
        10 PRINT "Comma test:", 1, 2, 3
        20 PRINT "Semicolon test:"; 1; 2; 3
        30 PRINT "Mixed test:", 1; 2, 3; 4
        40 PRINT "Empty line below:"
        50 PRINT
        60 PRINT "Concatenation: "; "Hello "; "World"; "!"
        70 END
    }
}
