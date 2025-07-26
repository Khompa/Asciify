use image::{ImageReader, DynamicImage, GrayImage};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let _gscale1 = r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~i!lI;:,"^`. "#;
    let _gscale2 = "@%#*+=-:. ";

    let img = ImageReader::open("src/images231.jpg")?.decode()?;
    img_to_ascii(img);
    Ok(())
}

fn img_to_ascii(img: DynamicImage) {

    let luma: GrayImage = img.into_luma8();
    let (width, height) = luma.dimensions();
    println!("Dimensions: {}x{}", width, height);
}