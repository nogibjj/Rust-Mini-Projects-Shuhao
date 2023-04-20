use std::io;
use image::{DynamicImage, GenericImageView, imageops::FilterType, Pixel};
use attohttpc::get;
use crossterm::{terminal, Result};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    println!("Image URL: ");
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("READ FAILED");

    let url = url.trim();
    let response = get(url).send().expect("REQUIRED FAILED");

    let img = image::load_from_memory(&response.bytes().expect("READ BYTE FAILED")).expect("LAOD FAILED");
    let img = resize_image_to_terminal_size(img)?;

    print_gray_image(&img);
    Ok(())
}

fn resize_image_to_terminal_size(mut img: DynamicImage) -> Result<DynamicImage> {
    let (terminal_width, terminal_height) = terminal::size()?;
    let (img_width, img_height) = img.dimensions();

    let aspect_ratio = img_width as f64 / img_height as f64;
    let new_width = (terminal_width * 3) as u32;
    let new_height = (((terminal_height*3) as f64 / 2.0) / aspect_ratio) as u32;

    img = img.resize(new_width, new_height, FilterType::Nearest);
    Ok(img)
}

fn print_gray_image(img: &DynamicImage) {
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_luma();
            let intensity = pixel[0] as f32 / 255.0;
            let character = match intensity {
                0.0..=0.2 => "  ",
                0.2..=0.4 => "░░",
                0.4..=0.6 => "▒▒",
                0.6..=0.8 => "▓▓",
                _ => "██",
            };
            print!("{}", character);
        }
        println!();
    }
}