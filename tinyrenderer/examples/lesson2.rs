use tgaimage::{TGAColor, TGAImage, TGAImageFormat};
use tinyrenderer::geometry::Vector2Int;
use tinyrenderer::{triangle, line};

const WHITE: TGAColor = TGAColor::new_rgba(255, 255, 255, 255);
const RED: TGAColor = TGAColor::new_rgba(255, 0, 0, 0);
const GREEN: TGAColor = TGAColor::new_rgba(0, 128, 0, 0);

fn main() {
    // first step
    let v1 = Vector2Int::new(100, 400);
    let v2 = Vector2Int::new(400, 400);
    let v3 = Vector2Int::new(250, 150);
    let v4 = Vector2Int::new(410, 100);
    let v5 = Vector2Int::new(525, 400);
    let v6 = Vector2Int::new(780, 100);
    let v7 = Vector2Int::new(260, 500);
    let v8 = Vector2Int::new(120, 420);
    let v9 = Vector2Int::new(360, 750);
    let mut image = TGAImage::new(800, 800, TGAImageFormat::RGB);

    triangle(v1, v2, v3, &WHITE, &mut image);
    triangle(v4, v5, v6, &RED, &mut image);
    triangle(v7, v8, v9, &GREEN, &mut image);
    line(120, 0, 120, 800, &WHITE, &mut image);
    image
        .write_tga_file("triangles.tga", false, true)
        .expect("Cannot write image");
}
