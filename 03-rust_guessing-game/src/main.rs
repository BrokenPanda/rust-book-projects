use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret: u8 = rand::thread_rng().gen_range(1..101);

    println!("--- Guessing Game ---");

    loop {
        println!("Enter your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            },
        }
    }
}
