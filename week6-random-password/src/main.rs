use rand::{thread_rng, Rng};

fn main() {
    let choices = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=";
    let mut rng = thread_rng();
    let password: String = (0..16)
        .map(|_| rng.gen_range(0..choices.len()))
        .map(|idx| choices.chars().nth(idx).unwrap())
        .collect();
    println!("{}", password);
}