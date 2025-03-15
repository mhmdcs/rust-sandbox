use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number! Press ctrl+c to exit the game :)");

    println!("Input your guess:");

    let secret_number = rand::thread_rng().gen_range(1..=100); // all numbers in rust default to
    // i32, but since we converted guess from string to u32, rust's compiler will infer that
    // secret_number should be u32 as well and thus it'll be u32 rather than the implicit default i32

    loop {
        let mut guess = String::new(); // returns an empty string instance

        io::stdin() // returns a Stdin instance
            .read_line(&mut guess) // returns a Result that can either be Ok or Err
            .expect("Failed to read line"); // handle the Result's Err

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
