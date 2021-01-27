//use std::io;
use rand::Rng;
use text_io::read;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number");
    println!("Please input your guess.");

    let guess: i32 = read!();
    
    if guess == secret_number {
        println!("You guessed right! {}", guess); 
    }
    else {
        println!("Guessed wrong! {}", guess);
    }

    
}
