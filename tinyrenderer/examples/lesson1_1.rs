/// Wireframe rendering
use tgaimage::{TGAColor, TGAImage, TGAImageFormat};
use tinyrenderer::{geometry, line, model::Model};

const WHITE: TGAColor = TGAColor::new_rgba(255, 255, 255, 255);

fn main() {
    let v = geometry::Vector2F32::new(1., 2.);
    let v2 = geometry::Vector2F32::new(0., 3.);

    println!("vector: {}", v);
    println!("vector: {}", v2 - v);

    let mut v = geometry::Vector3F32::new(1., 2., 3.);

    println!("normalized: {}", v.norm_f32());
    println!("normalized: {}", v.norm_f64());
    // println!("normalized: {}", v * 1.0f32);
    v.normalize_default();
    println!("normalized: {}", v);

    let width = 800u32;
    let height = 800u32;
    let model = Model::new("african_head.obj").unwrap();
    let mut image = TGAImage::new(width, height, TGAImageFormat::RGB);

    for i in 0..model.n_faces() {
        let face = model.face(i);

        for j in 0..3 {
            let v0 = model.vert(face[j] as usize);
            let v1 = model.vert(face[(j + 1) % 3] as usize);

            let x0 = (v0.get_x() + 1.) * width as f32 / 2.;
            let y0 = (v0.get_y() + 1.0) * height as f32 / 2.;
            let x1 = (v1.get_x() + 1.) * width as f32 / 2.;
            let y1 = (v1.get_y() + 1.) * height as f32 / 2.;

            line(
                x0 as i32, y0 as i32, x1 as i32, y1 as i32, WHITE, &mut image,
            );
        }
    }

    image
        .write_tga_file("africa.tga", true, true)
        .expect("Cannot write file");
}
