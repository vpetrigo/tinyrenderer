use tgaimage::{TGAColor, TGAImage};

use crate::geometry::{Vector2, Vector2Int, Vector3F32};

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

fn barycentric(points: &[Vector2Int; 3], point: Vector2Int) -> Vector3F32 {
    let u = Vector3F32::new(
        (points[2].get_x() - points[0].get_x()) as f32,
        (points[1].get_x() - points[0].get_x()) as f32,
        (points[0].get_x() - point.get_x()) as f32,
    ) ^ Vector3F32::new(
        (points[2].get_y() - points[0].get_y()) as f32,
        (points[1].get_y() - points[0].get_y()) as f32,
        (points[0].get_y() - point.get_y()) as f32,
    );

    if u.get_z().abs() < 1.0 {
        return Vector3F32::new(-1.0, 1.0, 1.0);
    }

    Vector3F32::new(
        1.0 - (u.get_x() + u.get_y()) / u.get_z(),
        u.get_y() / u.get_z(),
        u.get_x() / u.get_z(),
    )
}

/// Fill triangle with calculating barycentric coordinates
/// for properly determine which pixels should be filled
/// Arguments:
/// * `v1` - Vertice of a triangle
/// * `v2` - Vertice of a triangle
/// * `v3` - Vertice of a triangle
/// * `color` - color to fill triangle with
/// * `image` - image to draw triangle in
pub fn triangle_barycentric(
    v1: Vector2Int,
    v2: Vector2Int,
    v3: Vector2Int,
    color: &TGAColor,
    image: &mut TGAImage,
) {
    let mut boundary_box_min = Vector2Int::new(
        (image.get_width() - 1) as i32,
        (image.get_height() - 1) as i32,
    );
    let mut boundary_box_max = Vector2Int::new(0, 0);
    let clamp = boundary_box_min;
    let points = [v1, v2, v3];

    for p in &points {
        *boundary_box_min.get_x_as_mut() = 0.max(boundary_box_min.get_x().min(p.get_x()));
        *boundary_box_min.get_y_as_mut() = 0.max(boundary_box_min.get_y().min(p.get_y()));
        *boundary_box_max.get_x_as_mut() =
            clamp.get_x().min(boundary_box_max.get_x().max(p.get_x()));
        *boundary_box_max.get_y_as_mut() =
            clamp.get_y().min(boundary_box_max.get_y().max(p.get_y()));
    }

    for x in boundary_box_min.get_x()..=boundary_box_max.get_x() {
        for y in boundary_box_min.get_y()..=boundary_box_max.get_y() {
            let bc_screen = barycentric(&points, Vector2::new(x, y));

            if bc_screen.get_x() >= 0.0 && bc_screen.get_y() >= 0.0 && bc_screen.get_z() >= 0.0 {
                image.set(x as u32, y as u32, color);
            }
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
