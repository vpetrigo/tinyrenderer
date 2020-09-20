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
    let y_step = if dy > 0 { 1 } else { -1 };

    for x in x0..=x1 {
        if steep {
            image.set(y as u32, x as u32, color);
        } else {
            image.set(x as u32, y as u32, color);
        }

        error2 += derror2;

        if error2 > dx {
            y += y_step;
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

fn triangle_vertices_sort(v1: &mut Vector2Int, v2: &mut Vector2Int, v3: &mut Vector2Int) {
    if v1.get_y() > v2.get_y() {
        v1.swap(v2);
    }

    if v1.get_y() > v3.get_y() {
        v1.swap(v3);
    }

    if v2.get_y() > v3.get_y() {
        v2.swap(v3);
    }
}

pub fn triangle(
    mut v1: Vector2Int,
    mut v2: Vector2Int,
    mut v3: Vector2Int,
    color: &TGAColor,
    image: &mut TGAImage,
) {
    triangle_vertices_sort(&mut v1, &mut v2, &mut v3);

    if v2.get_y() == v3.get_y() {
        fill_flat_triangle(v1, v2, v3, color, image);
    } else if v1.get_y() == v2.get_y() {
        // fill top flat triangle
        fill_flat_triangle(v3, v1, v2, color, image);
    } else {
        // split to bottom and flat triangles and fill
        let v4 = Vector2::new(
            (v1.get_x() as f32
                + ((v2.get_y() - v1.get_y()) as f32 / (v3.get_y() - v1.get_y()) as f32)
                    * (v3.get_x() - v1.get_x()) as f32) as i32,
            v2.get_y(),
        );

        fill_flat_triangle(v1, v2, v4, color, image);
        fill_flat_triangle(v3, v2, v4, color, image);
    }
}

#[derive(Debug)]
struct SlopeParameters {
    dx: i32,
    dy: i32,
    is_steep: bool,
    x_step: i32,
    y_step: i32,
}

impl SlopeParameters {
    fn new(dx: i32, dy: i32, is_steep: bool, x_step: i32, y_step: i32) -> Self {
        SlopeParameters {
            dx,
            dy,
            is_steep,
            x_step,
            y_step,
        }
    }
}

fn fill_flat_triangle(
    v1: Vector2Int,
    v2: Vector2Int,
    v3: Vector2Int,
    color: &TGAColor,
    image: &mut TGAImage,
) {
    let mut slope1 = (v1.get_x(), v1.get_y());
    let mut slope2 = slope1;

    let slope1_params = get_slope_params(v1.get_x(), v1.get_y(), v2.get_x(), v2.get_y());
    let slope2_params = get_slope_params(v1.get_x(), v1.get_y(), v3.get_x(), v3.get_y());

    let mut error_slope1 = 2 * slope1_params.dy - slope1_params.dx;
    let mut error_slope2 = 2 * slope2_params.dy - slope2_params.dx;
    line(slope1.0, slope1.1, slope2.0, slope2.1, color, image);

    for _ in 0..=slope1_params.dx {
        println!("slope1: {:?} - slope2: {:?}", slope1, slope2);
        line(slope1.0, slope1.1, slope2.0, slope2.1, color, image);
        next_slope_point(&mut slope1, &mut error_slope1, &slope1_params);

        while slope2.1 != slope1.1 {
            next_slope_point(&mut slope2, &mut error_slope2, &slope2_params);
        }
    }
}

fn get_slope_params(x0: i32, y0: i32, x1: i32, y1: i32) -> SlopeParameters {
    let dx = (x0 - x1).abs();
    let dy = (y0 - y1).abs();
    let step_x = (x1 - x0).signum();
    let step_y = (y1 - y0).signum();

    if dx < dy {
        SlopeParameters::new(dy, dx, true, step_x, step_y)
    } else {
        SlopeParameters::new(dx, dy, false, step_x, step_y)
    }
}

fn next_slope_point(slope: &mut (i32, i32), slope_error: &mut i32, slope_params: &SlopeParameters) {
    while *slope_error >= 0 {
        if slope_params.is_steep {
            slope.0 += slope_params.x_step;
        } else {
            slope.1 += slope_params.y_step;
        }

        *slope_error -= slope_params.dx * 2;
    }

    if slope_params.is_steep {
        slope.1 += slope_params.y_step;
    } else {
        slope.0 += slope_params.x_step;
    }

    *slope_error += slope_params.dy * 2;
}
