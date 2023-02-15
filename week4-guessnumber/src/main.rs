use rand::Rng;
use std::io::BufRead;

fn main() {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1..101);
    println!("Guess a number between 1 and 100");
    for line in std::io::stdin().lock().lines() {
        let parsed = line.ok().as_deref().map(str::parse::<i64>);
        if let Some(Ok(guess)) = parsed {
            match guess {
                _ if guess < random => println!("Too low"),
                _ if guess > random => println!("Too high"),
                _ => {
                    println!("That's right");
                    break;
                }
            }
        }
    }
}