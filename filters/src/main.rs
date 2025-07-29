use std::io::{self, Write};
use image::ImageReader;
mod filters;
use filters::img_to_ascii;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ask if user wants colour or not
    let colour_choice = loop {
        print!("Do you want coloured ascii? (y/n): ");
        io::stdout().flush()?;                           // necessary in order to print before reading input       
        let mut input = String::new();           // variable to store input
        io::stdin().read_line(&mut input)?;         // read input
        let trimmed = input.trim().to_string(); // trim away the newline from input and convert to owned string
        
        // proceed if input is valid, loop otherwise
        if trimmed == "y" || trimmed == "n" {
            break trimmed; // return the valid choice
        }
        println!("Not a valid response. It's a simple question...");
    };
    
    let img = ImageReader::open("src/test_images/selfie.jpg")?.decode()?; //load image. TODO: ask for path as input instead of a set image.
    img_to_ascii(img, &colour_choice);
    Ok(())
}


