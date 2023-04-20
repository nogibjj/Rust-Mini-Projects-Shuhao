use std::io;
use std::f64;
use std::cmp::Ordering;

fn main() {
    println!("Please input the coefficients of the Quadratic equation a(x^2), b(x), c(constant) (separated by space):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("ERROR");

    let coefficients: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if coefficients.len() != 3 {
        println!("Please input 3 cofficient (0 for empty)");
        return;
    }

    let (a, b, c) = (coefficients[0], coefficients[1], coefficients[2]);

    if a == 0.0 {
        println!("cof a must not be 0");
        return;
    }

    let delta = b * b - 4.0 * a * c;

    match delta.partial_cmp(&0.0).unwrap() {
        Ordering::Less => println!("Equation have no real roots"),
        Ordering::Equal => {
            let x = -b / (2.0 * a);
            println!("Equation have one real root: x = {:.2}", x);
        }
        Ordering::Greater => {
            let x1 = (-b + delta.sqrt()) / (2.0 * a);
            let x2 = (-b - delta.sqrt()) / (2.0 * a);
            println!("Equation has two real roots: x1 = {:.2}, x2 = {:.2}", x1, x2);
        }
    }
}