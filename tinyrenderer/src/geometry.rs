use num;
use num::NumCast;
use num_traits::{Float, Num};
use std::default::Default;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Mul, Sub};

#[derive(Default, Copy, Clone)]
pub struct XYVector2<T: Num + Copy + Clone> {
    x: T,
    y: T,
}

#[derive(Default, Copy, Clone)]
pub struct UVVector2<T: Num + Copy + Clone> {
    u: T,
    v: T,
}

#[repr(C)]
pub union Vector2Repr<T: Num + Copy + Clone> {
    uvvector: UVVector2<T>,
    xyvector: XYVector2<T>,
    raw: [T; 2],
}

pub struct Vector2<T: Num + Copy + Clone> {
    repr: Vector2Repr<T>,
}

impl<T: Num + Copy + Clone> Vector2<T> {
    pub fn new(u: T, v: T) -> Self {
        Vector2 {
            repr: Vector2Repr {
                uvvector: UVVector2 { u, v },
            },
        }
    }
}

impl<T: Num + Default + Copy + Clone> Default for Vector2<T> {
    fn default() -> Self {
        Vector2 {
            repr: Vector2Repr {
                uvvector: UVVector2::default(),
            },
        }
    }
}

impl<T: Num + Copy + Clone> Mul for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        unsafe {
            Vector2::<T>::new(
                self.repr.uvvector.u * rhs.repr.uvvector.u,
                self.repr.uvvector.v * rhs.repr.uvvector.v,
            )
        }
    }
}

impl<T: Num + Copy + Clone> Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        unsafe {
            Vector2::<T>::new(
                self.repr.uvvector.u + rhs.repr.uvvector.u,
                self.repr.uvvector.v + rhs.repr.uvvector.v,
            )
        }
    }
}

impl<T: Num + Copy + Clone> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe {
            Vector2::<T>::new(
                self.repr.uvvector.u - rhs.repr.uvvector.u,
                self.repr.uvvector.v - rhs.repr.uvvector.v,
            )
        }
    }
}

impl<T: Display + Num + Copy + Clone> Display for Vector2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        unsafe {
            let x = self.repr.xyvector.x;
            let y = self.repr.xyvector.y;

            write!(f, "({}, {})", x, y)
        }
    }
}

pub type Vector2F32 = Vector2<f32>;
pub type Vector2Int = Vector2<i32>;

#[derive(Default, Copy, Clone)]
pub struct XYVector3<T: Num + NumCast + Copy + Clone> {
    x: T,
    y: T,
    z: T,
}

#[derive(Default, Copy, Clone)]
pub struct UVVector3<T: Num + NumCast + Copy + Clone> {
    vert: T,
    uv: T,
    norm: T,
}

#[repr(C)]
pub union Vector3Repr<T: Num + NumCast + Copy + Clone> {
    uvvector: UVVector3<T>,
    xyzvector: XYVector3<T>,
    raw: [T; 3],
}

pub struct Vector3<T: Num + NumCast + Copy + Clone> {
    repr: Vector3Repr<T>,
}

impl<T: Num + NumCast + Copy + Clone> Vector3<T> {
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
}

impl<T: Num + NumCast + Default + Copy + Clone> Default for Vector3<T> {
    fn default() -> Self {
        Vector3 {
            repr: Vector3Repr {
                xyzvector: XYVector3::default(),
            },
        }
    }
}

impl<T: Num + NumCast + Copy + Clone> Mul for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        unsafe {
            Vector3::<T>::new(
                self.repr.xyzvector.x * rhs.repr.xyzvector.x,
                self.repr.xyzvector.y * rhs.repr.xyzvector.y,
                self.repr.xyzvector.z * rhs.repr.xyzvector.z,
            )
        }
    }
}

impl<T, U> Mul<U> for Vector3<T>
where
    T: Num + NumCast + Copy + Clone,
    U: Float,
{
    type Output = U;

    fn mul(self, rhs: U) -> Self::Output {
        unsafe {
            num::cast::<T, U>(self.repr.xyzvector.x).unwrap() * rhs
                + num::cast::<T, U>(self.repr.xyzvector.y).unwrap() * rhs
                + num::cast::<T, U>(self.repr.xyzvector.z).unwrap() * rhs
        }
    }
}

impl<T: Num + NumCast + Copy + Clone> Add for Vector3<T> {
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

impl<T: Num + NumCast + Copy + Clone> Sub for Vector3<T> {
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

impl<T: Display + Num + NumCast + Copy + Clone> Display for Vector3<T> {
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
