use std::io;

fn main() {
    println!("Please input temperature followed by unit (C or F) eg: 36 C: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("ERROR");

    let input_parts: Vec<&str> = input.trim().split_whitespace().collect();

    if input_parts.len() != 2 {
        println!("Please input correct format");
        return;
    }

    let temperature: f64 = match input_parts[0].parse() {
        Ok(temp) => temp,
        Err(_) => {
            println!("Invalid input, use number");
            return;
        }
    };

    match input_parts[1].to_uppercase().as_str() {
        "C" => {
            let fahrenheit = (temperature * 9.0 / 5.0) + 32.0;
            println!("{:.2} C {:.2} F", temperature, fahrenheit);
        }
        "F" => {
            let celsius = (temperature - 32.0) * 5.0 / 9.0;
            println!("{:.2} F = {:.2} C", temperature, celsius);
        }
        _ => println!("Invlid Unit, use C or F."),
    }
}