use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    loop {
        println!("\nPlease, input your guess.");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");        
        println!("You guessed: {guess}");
        let guess: u32 =  match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                print!("Congrats!!!");
                break;
            },
            Ordering::Greater => print!("Too high!"),
        }   
    }
}
