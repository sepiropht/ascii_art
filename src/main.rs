extern crate image;

use image::GenericImageView;
use image::FilterType;
fn main() {
    let ascii = String::from("`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$");
    let mut res = String::from("");
    // Use the open function to load an image from a Path.
    //     // ```open``` returns a `DynamicImage` on success.
    let img = image::open("test.jpg").unwrap();
    //let mut matrix = vec![];
    //
    //             // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());
    //
    //                     // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());
    let img = img.resize(100, 100, FilterType::Nearest);
    //
    // Write the contents of this image to the Writer in PNG format.
    for pixel in img.pixels() {
        let rgba = pixel.2.data;
        let average : u32 = rgba.iter().take(3).map(|n| *n as u32).sum();
        let average = average / 3;
        let position : f32 = (average as f32) / (255 as f32);
        let position = position * (ascii.len() as f32);
        let position = position as usize;
        //println!("average {} position {}", average , position);
        let dig = ascii.chars().nth(position).unwrap();
        res = res +  &dig.to_string() + &dig.to_string() + &dig.to_string();

    }
    println!("{}", res);
}
