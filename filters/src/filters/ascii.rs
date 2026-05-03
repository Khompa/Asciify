use image::{DynamicImage, GrayImage, RgbImage};

pub fn img_to_ascii(img: DynamicImage, chars: &str, step_size: usize) -> (Vec<(u8, u8, u8)>, Vec<char>, usize) {

    // different grayscale lengths
    let gscale = if chars.is_empty() {
        r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~i!lI;:,"^`. "#
    } else {
        chars
    };

    let luma: GrayImage = img.clone().into_luma8();    // convert image to grayscale
    let rgb: RgbImage = img.into_rgb8();               // rgb version for colouring the ascii
    let (width, height) = luma.dimensions(); // get dimensions of the image


    let cols = width as usize/step_size; // calculate how many ascii characters we will have in width¨
    let rows = height as usize/step_size; // calculate how many ascii characters we will have in height
    let total_blocks = cols * rows;              // calculate total amount of ascii characters

    let mut pixel_array: Vec<char> = vec![' '; total_blocks]; // array to store the ascii values of the pixels
    let mut rgb_array: Vec<(u8, u8, u8)> = vec![(0, 0, 0); total_blocks]; // array to store the rgb values of the pixels


    // iterate by step_size pixels in height and in width
    for block_y in (0..height).step_by(step_size as usize) {
        for block_x in (0..width).step_by(step_size as usize) {

            if (block_x as usize/ step_size) >= cols || (block_y as usize/ step_size) >= rows {
                continue; // skip blocks that exceed the image dimensions
            }

            let mut total_luminosity = 0u32; // variable to store the luminosity of each pixel
            let mut total_r = 0u32;          // variable to store the red value of each pixel
            let mut total_g = 0u32;          // variable to store the green value of each pixel
            let mut total_b = 0u32;          // variable to store the blue value of each pixel
            let mut pixel_count = 0u32;      // variable to store the amount of pixels iterated over

            // iterate over each pixel in the 8x8 block
            for y in block_y..std::cmp::min(block_y + step_size as u32, height) {
                for x in block_x..std::cmp::min(block_x + step_size as u32, width) {

                    let pixel = luma.get_pixel(x, y);  // fetch the pixel
                    let pixel_rgb =rgb.get_pixel(x, y); // rgb version

                    total_r += pixel_rgb[0] as u32;      // store the red value of the pixel
                    total_g += pixel_rgb[1] as u32;      // store the green value of the pixel
                    total_b += pixel_rgb[2] as u32;      // store the blue value of the pixel
                    total_luminosity += pixel[0] as u32; //store the luminosity of the pixel
                    pixel_count +=1;                     // increase pixel_count
                }
            }

            let avg_luminosity = (total_luminosity/pixel_count) as u8; // calculate avg luminosity of pixels in block
            let avg_r = (total_r/pixel_count) as u8;                   // avg r value of pixels
            let avg_g = (total_g/pixel_count) as u8;                   // avg g value of pixels
            let avg_b = (total_b/pixel_count) as u8;                   // avg b value of pixels

            let index = ((255 - avg_luminosity) as usize * (gscale.len()-1)) / 255; // get the ascii_value corresponding to the avg luminosity
            let vec_index = (block_y as usize/ step_size) * cols + (block_x as usize/ step_size); // calculate the index in the ascii and rgb array
            
            // print!("\x1b[38;2;{};{};{}m{}\x1b[0m", avg_r, avg_g, avg_b, _gscale2.chars().nth(index).unwrap_or(' ')); // print the ascii in rgb colors
            rgb_array[vec_index] = (avg_r, avg_g, avg_b); // store the rgb values in the rgb array

            let char_value = gscale.chars().nth(index).unwrap_or(' ');
            pixel_array[vec_index] = char_value; // store the ascii value in the pixel array
            

        }
        // println!();      // go to new line
    }

    (rgb_array, pixel_array, cols) // return the rgb and ascii arrays along with the number of columns
}