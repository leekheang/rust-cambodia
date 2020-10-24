extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let screct_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", screct_number);
    println!("Please input your guess.");
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    println!("You guessed: {}", guess);

    let guess: u32 = guess.trim().parse().expect("Please Type a number");

    match guess.cmp(&screct_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win")
    }
}

