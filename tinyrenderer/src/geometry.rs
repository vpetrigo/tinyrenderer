use core::mem;
use std::default::Default;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, BitXor, Mul, MulAssign, Sub};

use num;
use num::cast::AsPrimitive;
use num::NumCast;
use num_traits::{Float, Num, ToPrimitive};

pub trait VectorTrait<T>: Copy + Clone + Num + NumCast + ToPrimitive + AsPrimitive<T>
where
    T: Copy + 'static,
{
}

macro_rules! impl_vector_trait {
    ($($t:ty)+) => {
        $( impl VectorTrait<$t> for $t {} )*
    };
}

impl_vector_trait!(i32 f32);

#[derive(Debug, Copy, Clone)]
pub struct Vector2<T: VectorTrait<T>> {
    x: T,
    y: T,
}

impl<T: VectorTrait<T>> Vector2<T> {
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
}

impl<T: VectorTrait<T> + Default> Default for Vector2<T> {
    fn default() -> Self {
        Vector2 {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T: VectorTrait<T>> Mul for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T: VectorTrait<T>> Mul<f32> for Vector2<T>
where
    f32: AsPrimitive<T>,
{
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(
            (self.x.to_f32().unwrap() * rhs).as_(),
            (self.y.to_f32().unwrap() * rhs).as_(),
        )
    }
}

impl<T: VectorTrait<T>> Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::<T>::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: VectorTrait<T>> Add<f32> for Vector2<T>
where
    f32: AsPrimitive<T>,
{
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.x + rhs.as_(), self.y + rhs.as_())
    }
}

impl<T: VectorTrait<T>> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::<T>::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Display + VectorTrait<T>> Display for Vector2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub type Vector2F32 = Vector2<f32>;
pub type Vector2Int = Vector2<i32>;

pub trait XAxis<T> {
    fn get_x(&self) -> T;

    fn x_as_mut_ref(&mut self) -> &mut T;
}

pub trait YAxis<T> {
    fn get_y(&self) -> T;

    fn y_as_mut_ref(&mut self) -> &mut T;
}

pub trait ZAxis<T> {
    fn get_z(&self) -> T;

    fn z_as_mut_ref(&mut self) -> &mut T;
}

pub trait XYAxis<T>: XAxis<T> + YAxis<T> {}

pub trait XYZAxis<T>: XAxis<T> + YAxis<T> + ZAxis<T> {}

impl<T> XAxis<T> for Vector2<T>
where
    T: VectorTrait<T>,
{
    fn get_x(&self) -> T {
        self.x
    }

    fn x_as_mut_ref(&mut self) -> &mut T {
        &mut self.x
    }
}

impl<T> YAxis<T> for Vector2<T>
where
    T: VectorTrait<T>,
{
    fn get_y(&self) -> T {
        self.y
    }

    fn y_as_mut_ref(&mut self) -> &mut T {
        &mut self.y
    }
}

impl<T> XYAxis<T> for Vector2<T> where T: VectorTrait<T> {}

pub(crate) trait NumMinMax {
    type Output;

    fn max_value() -> Self::Output;
    fn min_value() -> Self::Output;
}

macro_rules! impl_num_min_max_trait {
    ($t:ty) => {
        impl NumMinMax for $t {
            type Output = Self;

            fn max_value() -> Self::Output {
                <$t>::MAX
            }

            fn min_value() -> Self::Output {
                <$t>::MIN
            }
        }
    };
}

impl_num_min_max_trait!(i32);
impl_num_min_max_trait!(f32);

#[derive(Copy, Clone, Default, Debug)]
pub struct Vector3<T>
where
    T: VectorTrait<T>,
{
    x: T,
    y: T,
    z: T,
}

impl<T> Vector3<T>
where
    T: VectorTrait<T> + AsPrimitive<f32> + AsPrimitive<f64>,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
    }

    pub fn new_from_array(coords: &[T; 3]) -> Self {
        Vector3 {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }

    fn get_sum_of_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn get_sum_of_squared_f32(&self) -> f32 {
        self.get_sum_of_squared().as_()
    }

    fn get_sum_of_squared_f64(&self) -> f64 {
        self.get_sum_of_squared().as_()
    }

    pub fn norm_f32(&self) -> f32 {
        self.get_sum_of_squared_f32().sqrt()
    }

    pub fn norm_f64(&self) -> f64 {
        self.get_sum_of_squared_f64().sqrt()
    }

    pub fn normalize(&mut self, l: f32)
    where
        Vector3<T>: MulAssign<f32>,
    {
        *self *= l / self.norm_f32();
    }

    pub fn normalize_default(&mut self)
    where
        Vector3<T>: MulAssign<f32>,
    {
        self.normalize(1.0f32)
    }
}

/// Dot product
impl<T> Mul for Vector3<T>
where
    T: VectorTrait<T> + AsPrimitive<f32> + AsPrimitive<f64>,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T, U> Mul<U> for Vector3<T>
where
    T: VectorTrait<T> + AsPrimitive<U> + AsPrimitive<f32> + AsPrimitive<f64>,
    U: Float + AsPrimitive<T>,
{
    type Output = Self;

    fn mul(self, rhs: U) -> Self::Output {
        Vector3::<T>::new(
            (rhs * self.x.as_()).as_(),
            (rhs * self.y.as_()).as_(),
            (rhs * self.z.as_()).as_(),
        )
    }
}

impl<T, U> MulAssign<U> for Vector3<T>
where
    T: VectorTrait<T> + AsPrimitive<U> + AsPrimitive<f32> + AsPrimitive<f64>,
    U: Float + AsPrimitive<T>,
{
    fn mul_assign(&mut self, rhs: U) {
        self.x = (rhs * self.x.as_()).as_();
        self.y = (rhs * self.y.as_()).as_();
        self.z = (rhs * self.z.as_()).as_();
    }
}

impl<T> Add for Vector3<T>
where
    T: VectorTrait<T> + AsPrimitive<f32> + AsPrimitive<f64>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::<T>::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub for Vector3<T>
where
    T: VectorTrait<T> + AsPrimitive<f32> + AsPrimitive<f64>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::<T>::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

/// Cross product
impl<T> BitXor for Vector3<T>
where
    T: VectorTrait<T> + AsPrimitive<f32> + AsPrimitive<f64>,
{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Vector3::<T>::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl<T> Display for Vector3<T>
where
    T: VectorTrait<T> + AsPrimitive<f32> + AsPrimitive<f64> + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub type Vector3F32 = Vector3<f32>;
pub type Vector3Int = Vector3<i32>;

impl<T> XAxis<T> for Vector3<T>
where
    T: VectorTrait<T>,
{
    fn get_x(&self) -> T {
        self.x
    }

    fn x_as_mut_ref(&mut self) -> &mut T {
        &mut self.x
    }
}

impl<T> YAxis<T> for Vector3<T>
where
    T: VectorTrait<T>,
{
    fn get_y(&self) -> T {
        self.y
    }

    fn y_as_mut_ref(&mut self) -> &mut T {
        &mut self.y
    }
}

impl<T> ZAxis<T> for Vector3<T>
where
    T: VectorTrait<T>,
{
    fn get_z(&self) -> T {
        self.z
    }

    fn z_as_mut_ref(&mut self) -> &mut T {
        &mut self.z
    }
}

impl<T> XYAxis<T> for Vector3<T> where T: VectorTrait<T> {}
impl<T> XYZAxis<T> for Vector3<T> where T: VectorTrait<T> {}

#[derive(Debug, Copy, Clone)]
pub struct UVMap<T: Num + Copy + Clone> {
    pub u: T,
    pub v: T,
    pub w: T,
}

pub type UVMapF32 = UVMap<f32>;

#[cfg(test)]
mod test_vector3 {
    use crate::geometry::{Vector3F32, XAxis, YAxis, ZAxis};

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
