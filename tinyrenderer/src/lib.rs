use tgaimage::{TGAColor, TGAImage};

use crate::geometry::{Vector2, Vector2Int};

pub mod geometry;
pub mod model;

pub fn line(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    color: &TGAColor,
    image: &mut TGAImage,
) {
    let steep = if (x0 - x1).abs() < (y0 - y1).abs() {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        true
    } else {
        false
    };

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;

    for x in x0..=x1 {
        if steep {
            image.set(y as u32, x as u32, color);
        } else {
            image.set(x as u32, y as u32, color);
        }

        error2 += derror2;

        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
}

pub fn triangle(
    mut v1: Vector2Int,
    mut v2: Vector2Int,
    mut v3: Vector2Int,
    color: &TGAColor,
    image: &mut TGAImage,
) {
    if v1.get_y() > v2.get_y() {
        v1.swap(&mut v2);
    }

    if v1.get_y() > v3.get_y() {
        v1.swap(&mut v3);
    }

    if v2.get_y() > v3.get_y() {
        v2.swap(&mut v3);
    }

    if v2.get_y() == v3.get_y() {
        // fill bottom flat triangle
        fill_bottom_flat_triangle(v1, v2, v3, color, image);
    } else if v1.get_y() == v2.get_y() {
        // fill top flat triangle
        fill_top_flat_triangle(v1, v2, v3, color, image);
    } else {
        // split to bottom and flat triangles and fill
        let v4 = Vector2::new(
            (v1.get_x() as f32
                + ((v2.get_y() - v1.get_y()) as f32 / (v3.get_y() - v1.get_y()) as f32)
                    * (v3.get_x() - v1.get_x()) as f32) as i32,
            v2.get_y(),
        );

        fill_bottom_flat_triangle(v1, v2, v4, color, image);
        fill_top_flat_triangle(v2, v4, v3, color, image);
    }
}

fn fill_bottom_flat_triangle(
    v1: Vector2Int,
    v2: Vector2Int,
    v3: Vector2Int,
    color: &TGAColor,
    image: &mut TGAImage,
) {
    let x_side1 = v2.get_x().min(v3.get_x());
    let x_side2 = v2.get_x().max(v3.get_x());

    line(v1.get_x(), v1.get_y(), v2.get_x(), v2.get_y(), color, image);
    line(v1.get_x(), v1.get_y(), v3.get_x(), v3.get_y(), color, image);

    for line_x in x_side1..=x_side2 {
        line(v1.get_x(), v1.get_y(), line_x, v2.get_y(), color, image);
    }
}

fn fill_top_flat_triangle(
    v1: Vector2Int,
    v2: Vector2Int,
    v3: Vector2Int,
    color: &TGAColor,
    image: &mut TGAImage,
) {
    let x_side1 = v1.get_x().min(v2.get_x());
    let x_side2 = v1.get_x().max(v2.get_x());

    line(v3.get_x(), v3.get_y(), v1.get_x(), v1.get_y(), color, image);
    line(v3.get_x(), v3.get_y(), v2.get_x(), v2.get_y(), color, image);

    for line_x in x_side1..=x_side2 {
        line(v3.get_x(), v3.get_y(), line_x, v1.get_y(), color, image);
    }
}
