use tgaimage::{TGAColor, TGAImage, TGAImageFormat};
use tinyrenderer::line;

const WHITE: TGAColor = TGAColor::new_rgba(255, 255, 255, 255);
const RED: TGAColor = TGAColor::new_rgba(255, 0, 0, 0);

fn main() {
    let mut image = TGAImage::new(100, 100, TGAImageFormat::RGB);

    image.set(52, 41, RED);
    // line(13, 20, 80, 40, WHITE, &mut image);
    line(13, 20, 80, 40, WHITE, &mut image);
    line(20, 13, 40, 80, RED, &mut image);
    line(80, 40, 13, 20, RED, &mut image);
    image
        .write_tga_file("output.tga", true, true)
        .expect("Cannot write file");
}
