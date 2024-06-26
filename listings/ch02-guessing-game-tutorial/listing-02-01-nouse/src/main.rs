// ANCHOR: all
// ANCHOR_END: io
// use std::io;         NOT imported
// ANCHOR: main
fn main() {
    // ANCHOR_END: main
    // ANCHOR: print
    println!("Guess the number!");

    println!("Please input your guess.");
    // ANCHOR_END: print

    // ANCHOR: string
    let mut guess = String::new();
    // ANCHOR_END: string

    // ANCHOR: read
    std::io::stdin()
        .read_line(&mut guess)
        // ANCHOR_END: read
        // ANCHOR: expect
        .expect("Failed to read line");
    // ANCHOR_END: expect

    // ANCHOR: print_guess
    println!("You guessed: {}", guess);
    // ANCHOR_END: print_guess
}
// ANCHOR: all
