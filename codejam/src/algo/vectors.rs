use num_bigint::BigInt;
use num_traits::*;
use std::convert::From;
use std::ops::{Add, Div, Mul, Neg, Sub};
//https://github.com/PistonDevelopers/vecmath/blob/master/src/lib.rs

use num_traits::*;

pub type Vector3<T> = [T; 3];
pub fn vec3_sub<T>(a: &Vector3<T>, b: &Vector3<T>) -> Vector3<T>
where
    T: Copy + Sub<T, Output = T>,
{
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
pub fn vec3_cross<T>(a: &Vector3<T>, b: &Vector3<T>) -> Vector3<T>
where
    T: Copy + Mul<T, Output = T> + Sub<T, Output = T>,
{
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn vec3_cross_ref<'a, T>(a: &'a Vector3<T>, b: &'a Vector3<T>) -> Vector3<T>
where
    &'a T: Mul<&'a T, Output = T>,
    T: Sub<T, Output = T>,
{
    [
        &a[1] * &b[2] - &a[2] * &b[1],
        &a[2] * &b[0] - &a[0] * &b[2],
        &a[0] * &b[1] - &a[1] * &b[0],
    ]
}
pub fn vec3_dot<T>(a: &Vector3<T>, b: &Vector3<T>) -> T
where
    T: Copy + Add<T, Output = T> + Mul<T, Output = T>,
{
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn vec3_dot_ref<'a, T>(a: &'a Vector3<T>, b: &'a Vector3<T>) -> T
where
    &'a T: Mul<&'a T, Output = T>,
    T: Add<T, Output = T>,
{
    &a[0] * &b[0] + &a[1] * &b[1] + &a[2] * &b[2]
}

pub fn vec3_normalized<T>(a: &Vector3<T>) -> Vector3<T>
where
    T: Copy + One + Float + Add<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    vec3_scale(a, vec3_inv_len(a))
}

pub fn vec3_inv_len<T>(a: &Vector3<T>) -> T
where
    T: Copy + One + Float + Add<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    let one = T::one();
    one / vec3_len(a)
}

/// Computes the length of vector.
pub fn vec3_len<T>(a: &Vector3<T>) -> T
where
    T: Copy + Float + Add<T, Output = T> + Mul<T, Output = T>,
{
    vec3_square_len(a).sqrt()
}

pub fn vec3_square_len<T>(a: &Vector3<T>) -> T
where
    T: Copy + Add<T, Output = T> + Mul<T, Output = T>,
{
    a[0] * a[0] + a[1] * a[1] + a[2] * a[2]
}

pub fn vec3_scale<T>(a: &Vector3<T>, b: T) -> Vector3<T>
where
    T: Copy + Mul<T, Output = T>,
{
    [a[0] * b, a[1] * b, a[2] * b]
}

pub fn vec3_cast<T, U>(a: &Vector3<T>) -> Vector3<U>
where
    T: Copy + NumCast,
    U: NumCast,
{
    [
        cast::<T, U>(a[0]).unwrap(),
        cast::<T, U>(a[1]).unwrap(),
        cast::<T, U>(a[2]).unwrap(),
    ]
}

pub fn vec3_cast_bigint<T>(a: &Vector3<T>) -> Vector3<BigInt>
where
    T: Copy,
    BigInt: From<T>,
{
    [BigInt::from(a[0]), BigInt::from(a[1]), BigInt::from(a[2])]
}
