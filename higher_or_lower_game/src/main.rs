use rand::prelude::*;
use std::io;

fn main() {
    let number_to_guess = rand::rng().random_range(1..101);
    let mut buffer = String::new();
    let mut guessed_number: i32;
    println!("Let's play a Game! I've just generated random number between 1 and 100. Guess it!");
    
    loop {
        io::stdin().read_line(&mut buffer);
        guessed_number = buffer.trim().parse::<i32>().unwrap();

        if guessed_number > number_to_guess{
            println!("Your number is too high. Try again!");
        }
        else if guessed_number < number_to_guess{
            println!("Your number is too low. Try again!");
        }
        else {
            println!("YOU WON!!! Congrats!");
            break;
        }
        buffer.clear();
    }
}
