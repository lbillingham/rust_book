use std::io;
use std::cmp::Ordering;
extern crate rand;
use rand::Rng;

fn main() {
    println!("Guess the integer number!");
    let min = 0;
    let max = 100;
    let secret_number = rand::thread_rng().gen_range(min + 1, max + 1);
    println!("the secret number is {}", secret_number);
    loop {
        println!("input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        println!("you guessed: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid guess, try an integer {}-{}", min, max);
                continue;
            },
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big"),
        }
    }
}
