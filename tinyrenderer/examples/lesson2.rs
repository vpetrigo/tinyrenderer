use rand::random;

use tgaimage::{TGAColor, TGAImage, TGAImageFormat};
use tinyrenderer::geometry::{Vector2Int, Vector3F32};
use tinyrenderer::model::Model;
use tinyrenderer::{triangle, triangle_barycentric};

const WHITE: TGAColor = TGAColor::new_rgba(255, 255, 255, 255);
const RED: TGAColor = TGAColor::new_rgba(255, 0, 0, 0);
const GREEN: TGAColor = TGAColor::new_rgba(0, 128, 0, 0);

fn main() {
    // first step (Triangles)
    let v1 = Vector2Int::new(100, 400);
    let v2 = Vector2Int::new(400, 400);
    let v3 = Vector2Int::new(250, 150);
    let v4 = Vector2Int::new(410, 100);
    let v5 = Vector2Int::new(525, 400);
    let v6 = Vector2Int::new(780, 100);
    let v7 = Vector2Int::new(260, 500);
    let v8 = Vector2Int::new(120, 420);
    let v9 = Vector2Int::new(360, 750);
    let v10 = Vector2Int::new(410, 410);
    let v11 = Vector2Int::new(320, 520);
    let v12 = Vector2Int::new(780, 410);
    let mut image = TGAImage::new(800, 800, TGAImageFormat::RGB);

    triangle(v1, v2, v3, &WHITE, &mut image);
    triangle(v4, v5, v6, &RED, &mut image);
    triangle(v7, v8, v9, &GREEN, &mut image);
    triangle(v10, v11, v12, &WHITE, &mut image);
    // triangle_barycentric(v1, v2, v3, &WHITE, &mut image);
    // triangle_barycentric(v4, v5, v6, &RED, &mut image);
    // triangle_barycentric(v7, v8, v9, &GREEN, &mut image);
    // triangle_barycentric(v10, v11, v12, &WHITE, &mut image);

    image
        .write_tga_file("triangles.tga", true, true)
        .expect("Cannot write image");
    // Second step
    plot_head();
}

fn plot_head() {
    let width = 800u32;
    let height = 800u32;
    let model = Model::new("african_head.obj").unwrap();
    let mut image = TGAImage::new(width, height, TGAImageFormat::RGB);
    let light_dir = Vector3F32::new(0., 0., -1.);

    println!("v #{} f #{}", model.n_verts(), model.n_faces());
    // plot random color head
    for i in 0..model.n_faces() {
        let face = model.face(i);
        let mut screen_coords = [Vector2Int::default(); 3];

        for j in 0..3 {
            let v0 = model.vert(face[j] as usize);
            *screen_coords[j].get_x_as_mut() = ((v0.get_x() + 1.0) * width as f32 / 2.0) as i32;
            *screen_coords[j].get_y_as_mut() = ((v0.get_y() + 1.0) * height as f32 / 2.0) as i32;
        }

        triangle_barycentric(
            screen_coords[0],
            screen_coords[1],
            screen_coords[2],
            &TGAColor::new_rgb(random(), random(), random()),
            &mut image,
        );
    }

    image
        .write_tga_file("african_clown.tga", true, true)
        .expect("Cannot write image");
    image.clear();
    // plot head with light
    for i in 0..model.n_faces() {
        let face = model.face(i);
        let mut screen_coords = [Vector2Int::default(); 3];
        let mut world_coords = [Vector3F32::default(); 3];

        for j in 0..3 {
            let v0 = model.vert(face[j] as usize);
            *screen_coords[j].get_x_as_mut() = ((v0.get_x() + 1.0) * width as f32 / 2.0) as i32;
            *screen_coords[j].get_y_as_mut() = ((v0.get_y() + 1.0) * height as f32 / 2.0) as i32;
            world_coords[j] = *v0;
        }

        let mut n = (world_coords[2] - world_coords[0]) ^ (world_coords[1] - world_coords[0]);

        n.normalize_default();
        let intensity = n * light_dir;

        if intensity > 0.0 {
            triangle(
                screen_coords[0],
                screen_coords[1],
                screen_coords[2],
                &TGAColor::new_rgb(
                    (intensity * u8::max_value() as f32) as u8,
                    (intensity * u8::max_value() as f32) as u8,
                    (intensity * u8::max_value() as f32) as u8,
                ),
                &mut image,
            );
        }
    }

    image
        .write_tga_file("africa_color.tga", true, true)
        .expect("Cannot write file");
    image.clear();
}
