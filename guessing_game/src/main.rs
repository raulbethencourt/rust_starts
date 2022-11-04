use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

/// .
///
/// # Panics
///
/// Panics if .
fn main() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1, 100);

    'l: loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let value: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue 'l,
        };

        // use Guess struct
        let guess: i32 = Guess::new(value).value();

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!!".red()),
            Ordering::Greater => println!("{}", "Too big!!".red()),
            Ordering::Equal => break 'l println!("{}", "You win!!".green()),
        }
    }
}

#[derive()]
pub struct Guess {
    value: i32,
}

impl Guess {
    /// Creates a new [`Guess`].
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    /// Returns the value of this [`Guess`].
    pub fn value(&self) -> i32 {
        self.value
    }
}
