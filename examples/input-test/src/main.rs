use basic_dsl::basic;

fn main() {
    println!("=== Testing INPUT Statement ===");

    basic! {
        10 PRINT "Testing various INPUT formats:"
        20 PRINT
        
        30 PRINT "Simple numeric input:"
        40 INPUT NUM
        50 PRINT "You entered number: "; NUM
        60 PRINT

        70 PRINT "String input with prompt:"
        80 INPUT "Enter your name", NAME
        90 PRINT "Hello, "; NAME; "!"
        100 PRINT

        110 PRINT "Age calculation:"
        120 INPUT "What year were you born", YEAR
        130 LET AGE = 2025 - YEAR
        140 PRINT "You are approximately "; AGE; " years old."
        150 PRINT

        160 PRINT "Simple math test:"
        170 INPUT "Enter first number", A
        180 INPUT "Enter second number", B
        190 PRINT A; "+"; B; "="; A + B
        200 PRINT A; "*"; B; "="; A * B
        210 PRINT

        220 PRINT "Test completed!"
        230 END
    }
}
