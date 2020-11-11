extern crate rand;
use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number between 1 and 10!");
    let secret = rand::thread_rng().gen_range(1, 11);

    loop {
        println!("Input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to parse input");

        let guess: i32 = guess.trim().parse().expect("Failed to parse input");

        if guess == secret {
            println!("Well done, you got it!");
            break;
        } else {
            println!("Try again!");
            if guess < secret {
                println!("Hint: Go higher.");
            } else {
                println!("Hint: Go lower.");
            }
        }
    }
}
