use std::cmp::Ordering;
use std::io::stdin;

use rand::Rng;

fn main() {
    println!("Welcome to the NumberGuessing game!");
    println!("Please choose a range for the secret number you want to guess.");
    println!("Enter the lower bound:");
    let mut lower_bound = String::new();
    stdin()
        .read_line(&mut lower_bound)
        .expect("Please type a number!");
    let lower_bound: u32 = lower_bound.trim().parse().expect("Please type a number!");
    println!("Enter the upper bound:");
    let mut upper_bound = String::new();
    stdin()
        .read_line(&mut upper_bound)
        .expect("Please type a number!");
    let upper_bound: u32 = upper_bound.trim().parse().expect("Please type a number!");
    let secret_number = rand::thread_rng().gen_range(lower_bound..=upper_bound);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("Please type a number!");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed: {}", guess);
        if guess < lower_bound {
            println!("Your guess is lower than the lower bound!");
            continue;
        }
        if guess > upper_bound {
            println!("Your guess is higher than the upper bound!");
            continue;
        }
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
