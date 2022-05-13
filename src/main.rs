use std::io;
use std::process;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing game!");

    println!("Guess a number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("wrong input");

        if guess.trim() == "quit" {process::exit(0x100)}

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed right!");
                break;
            },
        }
    }
}