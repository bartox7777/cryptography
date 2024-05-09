// Algorytm	najmniej	znaczącego	bitu	polega	na	zmianie	najmniej	znaczącego	bitu	słowa
// opisującego	dany	piksel,	przy	czym	wynik	zależy	ściśle	od	liczby	bitów	przeznaczonych
// do	opisu	pojedynczego	piksela

use image::GenericImageView;
use image::{self, GenericImage};

// Wczytanie obrazu
fn read_image(file_name: &str) -> image::DynamicImage {
    image::open(file_name).unwrap()
}

// Zapisanie tekstu w obrazie (najmniej znaczący bit) i zapisanie obrazu
fn write_text_to_image(mut image: image::DynamicImage, text: &str) -> image::DynamicImage {
    let mut text = text.chars().collect::<Vec<char>>();
    text.push('\0');
    let text = text.iter().map(|c| *c as u8).collect::<Vec<u8>>();
    // divide image into chunks of 8 pixels
    // for chunk in image.pixels().collect::<Vec<_>>().chunks_mut(8) {
    //     let letter = text.iter().next();
    //     if let None = letter {
    //         break;
    //     }
    //     let letter = *letter.unwrap();
    //     for shift in 0..8 {
    //         chunk[shift].2 .0[0] = (chunk[shift].2 .0[0] & 0b11111110) | ((letter >> shift) & 1);
    //     }
    // }
    for i in 0..text.len() {
        for shift in 0..8 {
            let y = (i * 8 + shift) / image.width() as usize;
            let x = (i * 8 + shift) % image.width() as usize;
            let mut pixel = image.get_pixel(x as u32, y as u32);
            pixel.0[0] = (pixel.0[0] & 0b11111110) | ((text[i] >> shift) & 1);
            image.put_pixel(x as u32, y as u32, pixel);
        }
    }
    return image;
    // iterate over bits in text vector
    // for every mutable byte in image
}

// Odczytanie tekstu z obrazu (najmniej znaczący bit)
fn read_text_from_image(image: &image::DynamicImage) -> String {
    let mut text = String::new();
    for chunk in image.pixels().collect::<Vec<_>>().chunks(8) {
        let mut letter = 0;
        for shift in 0..8 {
            letter |= (chunk[shift].2 .0[0] & 1) << shift;
        }
        text.push(letter as char);
        if letter == 0 {
            break;
        }
    }
    text
}

fn main() {
    let image = read_image("lena.png");
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Lorem ipsum dolor sit amet, consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Lorem ipsum dolor sit amet, consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let image = write_text_to_image(image, text);
    image.save("image_with_text.png").unwrap();
    let image = read_image("image_with_text.png");
    let text = read_text_from_image(&image);
    println!("{}", text);
}
