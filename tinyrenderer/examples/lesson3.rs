use tgaimage::{TGAColor, TGAImage, TGAImageFormat};
use tinyrenderer::geometry::{Vector3F32, Vector3Int};
use tinyrenderer::model::Model;
use tinyrenderer::triangle_barycentric_zbuf;

fn main() {
    plot_head();
}

fn plot_head() {
    let width = 800u32;
    let height = 800u32;
    let mut model = Model::new("african_head.obj").unwrap();
    let mut image = TGAImage::new(width, height, TGAImageFormat::RGB);
    let light_dir = Vector3F32::new(0., 0., -1.);
    let mut z_buffer = [f32::NEG_INFINITY; 800 * 800];

    model
        .load_texture("african_head_diffuse.tga")
        .expect("Cannot load model texture");
    println!(
        "v #{} f #{}, vt# {}, vn# {}",
        model.n_verts(),
        model.n_faces(),
        model.n_textures(),
        model.n_normals()
    );
    // plot head with light and z-buffer
    for i in 0..model.n_faces() {
        let face = model.face(i);
        let mut screen_coords = [Vector3Int::default(); 3];
        let mut world_coords = [Vector3F32::default(); 3];

        for j in 0..3 {
            let v0 = model.vert(face[j] as usize);
            *screen_coords[j].get_x_as_mut() =
                ((v0.get_x() + 1.0) * width as f32 / 2.0 + 0.5) as i32;
            *screen_coords[j].get_y_as_mut() =
                ((v0.get_y() + 1.0) * height as f32 / 2.0 + 0.5) as i32;
            *screen_coords[j].get_z_as_mut() = (v0.get_z()) as i32;
            world_coords[j] = *v0;
        }

        let mut n = (world_coords[2] - world_coords[0]) ^ (world_coords[1] - world_coords[0]);

        n.normalize_default();
        let intensity = n * light_dir;

        if intensity > 0.0 {
            triangle_barycentric_zbuf(
                screen_coords[0],
                screen_coords[1],
                screen_coords[2],
                &mut z_buffer,
                &TGAColor::new_rgb(
                    (intensity * u8::MAX as f32) as u8,
                    (intensity * u8::MAX as f32) as u8,
                    (intensity * u8::MAX as f32) as u8,
                ),
                &mut image,
            );
        }
    }

    image
        .write_tga_file("africa_color.tga", true, true)
        .expect("Cannot write file");
    image.clear();

    let _texture_diffuse = TGAImage::read_tga_file("african_head_diffuse.tga").unwrap();
}
