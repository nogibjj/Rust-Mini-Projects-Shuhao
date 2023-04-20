use std::io;
use std::collections::HashMap;

fn main() {
    let mut movie_map = HashMap::new();
    movie_map.insert("action", "The Dark Knight");
    movie_map.insert("comedy", "Superbad");
    movie_map.insert("romance", "The Notebook");
    movie_map.insert("sci-fi", "Inception");
    movie_map.insert("drama", "The Shawshank Redemption");

    println!("Input keywords: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("FAILED");

    let keywords: Vec<&str> = input.trim().split_whitespace().collect();

    if keywords.is_empty() {
        println!("KEYWORD EMPTY");
        return;
    }

    println!("These are movies you may want to watch tonight:");

    for keyword in keywords {
        if let Some(movie) = movie_map.get(keyword) {
            println!("{}: {}", keyword, movie);
        } else {
            println!("{}: {}", keyword, "Star Wars");
        }
    }
}