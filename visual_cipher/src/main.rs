// Implementacja najprostszej wersji algorytmu, dla obrazu czarno-białego 100x100 pikseli,
// podział na dwa udziały, a po złożeniu udziałów brak korekcji zniekształcenia formatu i
// pozostawienie zaszumienia.

use image::{GrayImage, Luma};

fn main() {
    let img = image::open("src/lena.png").unwrap();
    let img = img.to_luma8(); // convert to grayscale
    let (width, height) = img.dimensions();
    let mut img1 = GrayImage::new(2 * width, height);
    let mut img2 = GrayImage::new(2 * width, height);

    for x in (0..2 * width).step_by(2) {
        for y in 0..height {
            let pixel = img.get_pixel(x / 2, y); // take pixel
            let pixel = pixel.0[0]; // take pixel value
            if pixel < 128 {
                // treat pixel as black
                // rand number 1..0
                if rand::random::<f64>() < 0.5 {
                    img1.put_pixel(x, y, Luma([0]));
                    img1.put_pixel(x + 1, y, Luma([255]));
                    img2.put_pixel(x, y, Luma([255]));
                    img2.put_pixel(x + 1, y, Luma([0]));
                } else {
                    img1.put_pixel(x, y, Luma([255]));
                    img1.put_pixel(x + 1, y, Luma([0]));
                    img2.put_pixel(x, y, Luma([0]));
                    img2.put_pixel(x + 1, y, Luma([255]));
                }
            } else {
                // treat pixel as white
                // rand number 1..0
                if rand::random::<f64>() < 0.5 {
                    img1.put_pixel(x, y, Luma([0]));
                    img1.put_pixel(x + 1, y, Luma([255]));
                    img2.put_pixel(x, y, Luma([0]));
                    img2.put_pixel(x + 1, y, Luma([255]));
                } else {
                    img1.put_pixel(x, y, Luma([255]));
                    img1.put_pixel(x + 1, y, Luma([0]));
                    img2.put_pixel(x, y, Luma([255]));
                    img2.put_pixel(x + 1, y, Luma([0]));
                }
            }
        }
    }

    img1.save("src/lena1.png").unwrap();
    img2.save("src/lena2.png").unwrap();
}
