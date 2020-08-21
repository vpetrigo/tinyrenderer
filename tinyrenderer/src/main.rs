extern crate tgaimage;

use tgaimage::{TGAColor, TGAImage, TGAImageFormat};

const WHITE: TGAColor = TGAColor::new_rgba(255, 255, 255, 255);
const RED: TGAColor = TGAColor::new_rgba(255, 0, 0, 0);

fn line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: TGAColor, image: &mut TGAImage) {
    let mut steep = false;

    if (x0 - x1).abs() < (y0 - y1).abs() {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    for x in x0..=x1 {
        let t = (x - x0) as f32 / (x1 - x0) as f32;
        let y = (y0 as f32 * (1. - t) + y1 as f32 * t) as i32;

        if steep {
            image.set(y as u32, x as u32, color);
        } else {
            image.set(x as u32, y as u32, color);
        }
    }
}

fn main() {
    let mut image = TGAImage::new(100, 100, TGAImageFormat::RGB);

    image.set(52, 41, RED);
    // line(13, 20, 80, 40, WHITE, &mut image);
    line(13, 20, 80, 40, WHITE, &mut image);
    line(20, 13, 40, 80, RED, &mut image);
    line(80, 40, 13, 20, RED, &mut image);
    // image.flip_vertically();
    image
        .write_tga_file("output.tga", true, true)
        .expect("Cannot write file");
}
