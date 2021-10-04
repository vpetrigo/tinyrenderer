use tgaimage::{TGAImage, TGAImageFormat};
use tinyrenderer::model::Model;

fn main() {
    let mut texture =
        TGAImage::read_tga_file("african_head_diffuse.tga").expect("Unable to read image");

    texture.write_tga_file("african_head_diffuse_tmp.tga", true, true);
    texture.clear();
}
