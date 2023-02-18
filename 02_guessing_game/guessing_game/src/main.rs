use rand::Rng;
use std::io;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Welcome to the guessing game");
    let random_number = rand::thread_rng().gen_range(1, 101); 
    println!("The Random Number is : {}", random_number);
    loop {
        println!("Please guess the number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("you guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }
        };
    }
}
