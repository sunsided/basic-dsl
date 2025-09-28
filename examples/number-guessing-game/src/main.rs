use basic_dsl::basic;

fn main() {
    println!("=== BASIC Number Guessing Game ===");
    println!("Think of a number between 1 and 100, and I'll try to guess it!");
    println!("Answer with 1 for higher, 2 for lower, or 3 for correct.");
    println!();

    basic! {
        10 PRINT "Starting binary search..."
        20 LET LOW = 1
        30 LET HIGH = 100
        40 LET GUESS = 50
        50 LET TRIES = 1

        100 PRINT "My guess is:", GUESS
        110 INPUT "Enter 1=Higher, 2=Lower, 3=Correct", ANSWER
        120 IF ANSWER = 3 THEN GOTO 1000
        130 IF ANSWER = 1 THEN GOTO 200
        140 IF ANSWER = 2 THEN GOTO 300
        150 PRINT "Please enter 1, 2, or 3"
        160 GOTO 100

        200 LET LOW = GUESS + 1
        210 GOTO 400
        
        300 LET HIGH = GUESS - 1
        310 GOTO 400

        400 IF LOW > HIGH THEN GOTO 900
        410 LET GUESS = (LOW + HIGH) / 2
        420 LET TRIES = TRIES + 1
        430 IF TRIES > 10 THEN GOTO 800
        440 GOTO 100

        800 PRINT "Hmm, I've made "; TRIES; " guesses. Are you sure you're playing fair?"
        810 GOTO 100

        900 PRINT "That's impossible! Please check your answers."
        910 GOTO 1100

        1000 PRINT "I got it! Your number was "; GUESS
        1010 PRINT "It took me "; TRIES; " tries using binary search."
        1020 PRINT "Thanks for playing!"

        1100 END
    }
}
