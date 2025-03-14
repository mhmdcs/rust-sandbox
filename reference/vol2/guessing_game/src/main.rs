use std::io;

fn main() {
    println!("Guess the number!");

    println!("Input your guess:");

    let mut guess = String::new(); // returns an empty string instance

    io::stdin() // returns a Stdin instance
        .read_line(&mut guess) // returns a Result that can either be Ok or Err
        .expect("Failed to read line"); // handle the Result's Err

    println!("You guessed: {}", guess);
}
