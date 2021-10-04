use std::ops::Neg;

use num::{One, Signed, Zero};

use tgaimage::{TGAColor, TGAImage};

use crate::geometry::{
    NumMinMax, Vector2, Vector2Int, Vector3Int, VectorTrait, XAxis, XYAxis, YAxis, ZAxis,
};
use crate::line::Line;
use crate::model::Model;
use crate::point::Point;

pub mod geometry;
pub mod line;
pub mod model;
pub mod point;

pub struct TriangleDef(pub Vector3Int, pub Vector3Int, pub Vector3Int);
pub struct TextureDef(pub Vector2Int, pub Vector2Int, pub Vector2Int);

pub struct PointBarycentricCoords {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

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

pub fn barycentric<T: VectorTrait<T> + Signed + Neg, U: XYAxis<T>, V: XYAxis<T>>(
    triangle_points: &[U; 3],
    point: V,
) -> Option<PointBarycentricCoords> {
    // calculate PA, AB and AC vector parameters in the A (point) basis
    let point_vec_x = point.get_x() - triangle_points[0].get_x();
    let point_vec_y = point.get_y() - triangle_points[0].get_y();
    let side_one_x = triangle_points[1].get_x() - triangle_points[0].get_x();
    let side_one_y = triangle_points[1].get_y() - triangle_points[0].get_y();
    let side_two_x = triangle_points[2].get_x() - triangle_points[0].get_x();
    let side_two_y = triangle_points[2].get_y() - triangle_points[0].get_y();
    let det = side_one_x * side_two_y - side_one_y * side_two_x;

    if det == T::zero() {
        return None;
    }
    // originally we have the following matrix:
    // | s1.x  s2.x |
    // | s1.y  s2.y |
    // and we need the adjugate:
    // |  s2.y  -s2.x |
    // | -s1.y   s1.x |
    let adj_coeffs = ((side_two_y, -side_two_x), (-side_one_y, side_one_x));
    let result = (
        adj_coeffs.0 .0 * point_vec_x + adj_coeffs.0 .1 * point_vec_y,
        adj_coeffs.1 .0 * point_vec_x + adj_coeffs.1 .1 * point_vec_y,
    );
    let det = det.to_f32().unwrap();
    let u = result.0.to_f32().unwrap() / det;
    let v = result.1.to_f32().unwrap() / det;
    let w = 1.0 - u - v;

    if u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
        Some(PointBarycentricCoords { u, v, w })
    } else {
        None
    }
}

fn boundary_box_setup<T>(points: &[Vector2<T>; 3], width: T, height: T) -> (Vector2<T>, Vector2<T>)
where
    T: VectorTrait<T> + NumMinMax<Output = T> + Ord + Zero + One,
{
    let mut boundary_box_min = Vector2::new(T::max_value(), T::max_value());
    let mut boundary_box_max = Vector2::new(T::min_value(), T::min_value());
    let clamp = Vector2::<T>::new((width - T::one()).as_(), (height - T::one()).as_());

    for point in points {
        *boundary_box_min.x_as_mut_ref() =
            T::zero().max(boundary_box_min.get_x().min(point.get_x()));
        *boundary_box_min.y_as_mut_ref() =
            T::zero().max(boundary_box_min.get_y().min(point.get_y()));
        *boundary_box_max.x_as_mut_ref() = clamp
            .get_x()
            .min(boundary_box_max.get_x().max(point.get_x()));
        *boundary_box_max.y_as_mut_ref() = clamp
            .get_y()
            .min(boundary_box_max.get_y().max(point.get_y()));
    }

    (boundary_box_min, boundary_box_max)
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
    let points = &[v1, v2, v3];
    let (boundary_box_min, boundary_box_max) =
        boundary_box_setup(points, image.get_width() as i32, image.get_height() as i32);

    for x in boundary_box_min.get_x()..=boundary_box_max.get_x() {
        for y in boundary_box_min.get_y()..=boundary_box_max.get_y() {
            if let Some(_) = barycentric(points, Vector2::new(x, y)) {
                image.set(x as u32, y as u32, color);
            }
        }
    }
}

pub fn triangle_barycentric_zbuf(
    v1: Vector3Int,
    v2: Vector3Int,
    v3: Vector3Int,
    zbuf: &mut [f32],
    color: &TGAColor,
    image: &mut TGAImage,
) {
    let points_2d = &[
        Vector2::new(v1.get_x(), v1.get_y()),
        Vector2::new(v2.get_x(), v2.get_y()),
        Vector2::new(v3.get_x(), v3.get_y()),
    ];
    let points = [v1, v2, v3];
    let (boundary_box_min, boundary_box_max) = boundary_box_setup(
        points_2d,
        image.get_width() as i32,
        image.get_height() as i32,
    );
    let mut z = 0.0;

    for x in boundary_box_min.get_x()..=boundary_box_max.get_x() {
        for y in boundary_box_min.get_y()..=boundary_box_max.get_y() {
            if let Some(bc_screen) = barycentric(&[v1, v2, v3], Vector3Int::new(x, y, z as i32)) {
                z = (points[0].get_z() as f32 * bc_screen.w
                    + points[1].get_z() as f32 * bc_screen.u
                    + points[2].get_z() as f32 * bc_screen.v) as f32;

                if zbuf[(x as u32 + y as u32 * image.get_width() as u32) as usize] < z {
                    zbuf[(x as u32 + y as u32 * image.get_width() as u32) as usize] = z;
                    image.set(x as u32, y as u32, color);
                }
            }
        }
    }
}

pub fn triangle_barycentric_zbuf_with_texture(
    triangle_def: TriangleDef,
    texture_def: TextureDef,
    zbuf: &mut [f32],
    image: &mut TGAImage,
    model: &Model,
    intensity: f32,
) {
    let points_2d = &[
        Vector2::new(triangle_def.0.get_x(), triangle_def.0.get_y()),
        Vector2::new(triangle_def.1.get_x(), triangle_def.1.get_y()),
        Vector2::new(triangle_def.2.get_x(), triangle_def.2.get_y()),
    ];
    let points = [triangle_def.0, triangle_def.1, triangle_def.2];
    let (boundary_box_min, boundary_box_max) = boundary_box_setup(
        points_2d,
        image.get_width() as i32,
        image.get_height() as i32,
    );

    for x in boundary_box_min.get_x()..=boundary_box_max.get_x() {
        for y in boundary_box_min.get_y()..=boundary_box_max.get_y() {
            if let Some(bc_screen) = barycentric(&points, Vector2Int::new(x, y)) {
                let z = (points[0].get_z() as f32 * bc_screen.w
                    + points[1].get_z() as f32 * bc_screen.u
                    + points[2].get_z() as f32 * bc_screen.v) as f32;

                let index = (x + y * image.get_width() as i32) as usize;
                if zbuf[index] < z {
                    zbuf[index] = z;
                    let uv_p = texture_def.0 * bc_screen.w
                        + texture_def.1 * bc_screen.u
                        + texture_def.2 * bc_screen.v;
                    let color = model.diffuse(uv_p);
                    image.set(x as u32, y as u32, &(color.unwrap() * intensity));
                }
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

fn fill_flat_triangle(
    v1: Vector2Int,
    v2: Vector2Int,
    v3: Vector2Int,
    color: &TGAColor,
    image: &mut TGAImage,
) {
    let slope1 = Line::new(
        Point::new(v1.get_x(), v1.get_y()),
        Point::new(v2.get_x(), v2.get_y()),
    );
    let slope2 = Line::new(
        Point::new(v1.get_x(), v1.get_y()),
        Point::new(v3.get_x(), v3.get_y()),
    );

    let y_range = if v1.get_y() < v2.get_y() {
        v1.get_y()..=v2.get_y()
    } else {
        v2.get_y()..=v1.get_y()
    };

    for y in y_range {
        let mut min_p: i32 = i32::MAX;
        let mut max_p: i32 = i32::MIN;

        for slope in &[slope1, slope2] {
            slope
                .points()
                .skip_while(|p| p.y != y)
                .take_while(|p| p.y == y)
                .for_each(|p| {
                    min_p = min_p.min(p.x);
                    max_p = max_p.max(p.x);
                });
        }

        for x in min_p..=max_p {
            image.set(x as u32, y as u32, color);
        }
    }
}

#[cfg(test)]
mod test_renderer_lib {
    use crate::barycentric;
    use crate::geometry::Vector3Int;

    #[test]
    fn test_barycentric() {
        let v1 = Vector3Int::new(0, 0, 0);
        let v2 = Vector3Int::new(0, 2, 0);
        let v3 = Vector3Int::new(2, 0, 0);
        let p1 = Vector3Int::new(1, 0, 0);
        let p2 = Vector3Int::new(0, 1, 0);
        let p3 = Vector3Int::new(0, 0, 0);
        let p4 = Vector3Int::new(0, 2, 0);
        let p5 = Vector3Int::new(2, 0, 0);

        if let Some(bc) = barycentric(&[v1, v2, v3], p1) {
            assert_eq!(bc.w == 0.5 && bc.u == 0., bc.v == 0.5);
        } else {
            panic!("Invalid barycentric calculation");
        }

        if let Some(bc) = barycentric(&[v1, v2, v3], p2) {
            assert_eq!(bc.w == 0.5 && bc.u == 0.5, bc.v == 0.);
        } else {
            panic!("Invalid barycentric calculation");
        }

        if let Some(bc) = barycentric(&[v1, v2, v3], p3) {
            assert_eq!(bc.w == 1. && bc.u == 0., bc.v == 0.);
        } else {
            panic!("Invalid barycentric calculation");
        }

        if let Some(bc) = barycentric(&[v1, v2, v3], p4) {
            assert_eq!(bc.w == 0. && bc.u == 1., bc.v == 0.);
        } else {
            panic!("Invalid barycentric calculation");
        }

        if let Some(bc) = barycentric(&[v1, v2, v3], p5) {
            assert_eq!(bc.w == 0. && bc.u == 0., bc.v == 1.);
        } else {
            panic!("Invalid barycentric calculation");
        }
    }
}
