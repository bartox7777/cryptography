// Algorytm	najmniej	znaczącego	bitu	polega	na	zmianie	najmniej	znaczącego	bitu	słowa
// opisującego	dany	piksel,	przy	czym	wynik	zależy	ściśle	od	liczby	bitów	przeznaczonych
// do	opisu	pojedynczego	piksela

use image;
use image::GenericImageView;

// Wczytanie obrazu
fn read_image(file_name: &str) -> image::DynamicImage {
    image::open(file_name).unwrap()
}

// Zapisanie tekstu w obrazie (najmniej znaczący bit) i zapisanie obrazu
fn write_text_to_image(image: &mut image::DynamicImage, text: &str) {
    let mut text = text.chars().collect::<Vec<char>>();
    text.push('\0');
    let text = text.iter().map(|c| *c as u8).collect::<Vec<u8>>();
    let mut text = text.iter().cycle();
    // divide image into chunks of 8 pixels
    for chunk in image.pixels().collect::<Vec<_>>().chunks_mut(8) {
        for shift in 0..8{
            chunk[shift].2.0 = 
        }
    }
}

// Odczytanie tekstu z obrazu (najmniej znaczący bit)
fn read_text_from_image(image: &image::DynamicImage) -> String {
    let mut text = Vec::new();
    for pixel in image.pixels() {
        for channel in pixel.2 .0.iter() {
            text.push(*channel & 1);
        }
    }
    let text = text
        .chunks(8)
        .map(|chunk| chunk.iter().fold(0, |acc, &bit| (acc << 1) | bit))
        .collect::<Vec<u8>>();
    let text = text.iter().map(|&c| c as char).collect::<String>();
    text.trim_end_matches('\0').to_string()
}

fn main() {
    let mut image = read_image("lena.png");
    let text = "Hello, World!";
    write_text_to_image(&mut image, text);
    image.save("image_with_text.png").unwrap();
    let image = read_image("image_with_text.png");
    let text = read_text_from_image(&image);
    println!("{}", text);
}
