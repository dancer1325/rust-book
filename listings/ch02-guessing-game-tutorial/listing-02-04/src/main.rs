// ANCHOR: here
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--
    // ANCHOR_END: here
    println!("Guess the number!");

    // Type inferred, i32 by default
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();      // Type inferred

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // ANCHOR: here

    println!("You guessed: {guess}");

    // Error, because String -- can NOT be compared with -- i32
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
// ANCHOR_END: here
