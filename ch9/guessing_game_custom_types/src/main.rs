extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

struct Guess {
    value: u32
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        // putting validations on `new`
        // note that, without `&self` we get a `static` method, `class` method, `w/e` method
        if value < 1 || value > 100 {
            panic!("Guess must be between 1 and 100, got {}.", value);
        }

        Guess {
            value: value,
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    // will now panic if the secret number is greater than 100
    let secret_number = Guess::new(rand::thread_rng().gen_range(1, 101));

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number.value()) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
