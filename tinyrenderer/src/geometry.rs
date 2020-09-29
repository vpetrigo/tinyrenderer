use core::mem;
use num;
use num::cast::AsPrimitive;
use num::NumCast;
use num_traits::{Float, Num, ToPrimitive};
use std::default::Default;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, BitXor, Mul, MulAssign, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vector2<T: Num + Copy + Clone> {
    x: T,
    y: T,
}

impl<T: Num + Copy + Clone> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x, y }
    }

    pub fn swap(&mut self, rhs: &mut Self) {
        mem::swap(&mut self.x, &mut rhs.x);
        mem::swap(&mut self.y, &mut rhs.y);
    }

    pub fn swap_xy(&mut self) {
        mem::swap(&mut self.x, &mut self.y);
    }

    pub fn get_x(&self) -> T {
        self.x
    }

    pub fn get_y(&self) -> T {
        self.y
    }

    pub fn get_x_as_mut(&mut self) -> &mut T {
        &mut self.x
    }

    pub fn get_y_as_mut(&mut self) -> &mut T {
        &mut self.y
    }
}

impl<T: Num + Default + Copy + Clone> Default for Vector2<T> {
    fn default() -> Self {
        Vector2 {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T: Num + Copy + Clone> Mul for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector2::<T>::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T: Num + Copy + Clone> Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::<T>::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Num + Copy + Clone> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::<T>::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Display + Num + Copy + Clone> Display for Vector2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub type Vector2F32 = Vector2<f32>;
pub type Vector2Int = Vector2<i32>;

// pub trait VectorTrait: Copy + Clone + Num + NumCast + ToPrimitive + AsPrimitive<T> + MulAssign {}

// macro_rules! vector_trait_def {
//     ($t:tt) => {
//         impl VectorTrait for $t {}
//     };
// }
//
// vector_trait_def!(i32);
// vector_trait_def!(f32);

#[derive(Copy, Clone, Default)]
pub struct Vector3<T>
where
    T: Num + NumCast + AsPrimitive<T> + Copy + Clone,
{
    x: T,
    y: T,
    z: T,
}

impl<T> Vector3<T>
where
    T: Num + NumCast + AsPrimitive<T> + AsPrimitive<f32> + AsPrimitive<f64> + Copy + Clone,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 {
            repr: Vector3Repr {
                xyzvector: XYVector3 { x, y, z },
            },
        }
    }

    fn get_sum_of_squared(&self) -> T {
        unsafe {
            self.repr.xyzvector.x * self.repr.xyzvector.x
                + self.repr.xyzvector.y * self.repr.xyzvector.y
                + self.repr.xyzvector.z * self.repr.xyzvector.z
        }
    }

    fn get_sum_of_squared_f32(&self) -> f32 {
        num::cast(self.get_sum_of_squared()).unwrap()
    }

    fn get_sum_of_squared_f64(&self) -> f64 {
        num::cast(self.get_sum_of_squared()).unwrap()
    }

    pub fn norm_f32(&self) -> f32 {
        self.get_sum_of_squared_f32().sqrt()
    }

    pub fn norm_f64(&self) -> f64 {
        self.get_sum_of_squared_f64().sqrt()
    }

    pub fn normalize(&mut self, l: f32) {
        *self *= l as f32 / self.norm_f32();
    }

    pub fn normalize_default(&mut self) {
        self.normalize(1.0f32)
    }

    pub fn get_x(&self) -> T {
        unsafe { self.repr.xyzvector.x }
    }

    pub fn get_y(&self) -> T {
        unsafe { self.repr.xyzvector.y }
    }

    pub fn get_z(&self) -> T {
        unsafe { self.repr.xyzvector.z }
    }
}

impl<T: Default + VectorTrait> Default for Vector3<T> {
    fn default() -> Self {
        Vector3 {
            repr: Vector3Repr {
                xyzvector: XYVector3::default(),
            },
        }
    }
}

/// Dot product
impl<T: VectorTrait> Mul for Vector3<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        unsafe {
            self.repr.xyzvector.x * rhs.repr.xyzvector.x
                + self.repr.xyzvector.y * rhs.repr.xyzvector.y
                + self.repr.xyzvector.z * rhs.repr.xyzvector.z
        }
    }
}

impl<T, U> Mul<U> for Vector3<T>
where
    T: VectorTrait,
    U: Float,
{
    type Output = Self;

    fn mul(self, rhs: U) -> Self::Output {
        unsafe {
            Vector3::<T>::new(
                num::cast::<U, T>(num::cast::<T, U>(self.repr.xyzvector.x).unwrap() * rhs).unwrap(),
                num::cast::<U, T>(num::cast::<T, U>(self.repr.xyzvector.y).unwrap() * rhs).unwrap(),
                num::cast::<U, T>(num::cast::<T, U>(self.repr.xyzvector.z).unwrap() * rhs).unwrap(),
            )
        }
    }
}

impl<T, U> MulAssign<U> for Vector3<T>
where
    T: VectorTrait,
    U: Float,
{
    fn mul_assign(&mut self, rhs: U) {
        unsafe {
            self.repr.xyzvector.x =
                num::cast::<U, T>(num::cast::<T, U>(self.repr.xyzvector.x).unwrap() * rhs).unwrap();
            self.repr.xyzvector.y =
                num::cast::<U, T>(num::cast::<T, U>(self.repr.xyzvector.y).unwrap() * rhs).unwrap();
            self.repr.xyzvector.z =
                num::cast::<U, T>(num::cast::<T, U>(self.repr.xyzvector.z).unwrap() * rhs).unwrap();
        }
    }
}

impl<T: VectorTrait> Add for Vector3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        unsafe {
            Vector3::<T>::new(
                self.repr.xyzvector.x + rhs.repr.xyzvector.x,
                self.repr.xyzvector.y + rhs.repr.xyzvector.y,
                self.repr.xyzvector.z + rhs.repr.xyzvector.z,
            )
        }
    }
}

impl<T: VectorTrait> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe {
            Vector3::<T>::new(
                self.repr.xyzvector.x - rhs.repr.xyzvector.x,
                self.repr.xyzvector.y - rhs.repr.xyzvector.y,
                self.repr.xyzvector.z - rhs.repr.xyzvector.z,
            )
        }
    }
}

/// Cross product
impl<T: VectorTrait> BitXor for Vector3<T> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe {
            Vector3::<T>::new(
                self.repr.xyzvector.y * rhs.repr.xyzvector.z
                    - self.repr.xyzvector.z * rhs.repr.xyzvector.y,
                self.repr.xyzvector.z * rhs.repr.xyzvector.x
                    - self.repr.xyzvector.x * rhs.repr.xyzvector.z,
                self.repr.xyzvector.x * rhs.repr.xyzvector.y
                    - self.repr.xyzvector.y * rhs.repr.xyzvector.x,
            )
        }
    }
}

impl<T: Display + VectorTrait> Display for Vector3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        unsafe {
            write!(
                f,
                "({}, {}, {})",
                self.repr.xyzvector.x, self.repr.xyzvector.y, self.repr.xyzvector.z
            )
        }
    }
}

pub type Vector3F32 = Vector3<f32>;
pub type Vector3Int = Vector3<i32>;

#[cfg(test)]
mod test_vector3 {
    use crate::geometry::Vector3F32;

    #[test]
    fn test_normalization() {
        let mut v = Vector3F32::new(3.0, 4.0, 5.0);
        let expected_sqrt = (3.0f32 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0).sqrt();

        assert_eq!(expected_sqrt, v.norm_f32());

        let expected = Vector3F32::new(
            3.0 as f32 / expected_sqrt,
            4.0 as f32 / expected_sqrt,
            5.0 as f32 / expected_sqrt,
        );

        v.normalize_default();
        assert!((expected.get_x() - v.get_x()).abs() < 0.05);
        assert!((expected.get_y() - v.get_y()).abs() < 0.05);
        assert!((expected.get_z() - v.get_z()).abs() < 0.05);
    }
}
