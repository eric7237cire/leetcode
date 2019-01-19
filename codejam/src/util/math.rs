use num_integer::Integer;
use num_traits::*;

pub fn int_div_ceil<T>(n1: T, n2: T) -> T
where
    T: Integer,
{
    debug_assert!(n1 > T::zero());
    debug_assert!(n2 > T::zero());
    T::one() + ((n1 - T::one()) / n2)
}

pub fn int_sub_us<T: Integer + Unsigned, U: Integer + Signed>(n1: T, n2: U) -> T
where
    T: NumCast,
    U: NumCast,
{
    cast::<U, T>(cast::<T, U>(n1).unwrap() - n2).unwrap()
}

pub fn int_sub<T, U, V>(n1: T, n2: U) -> V
where
    T: NumCast,
    U: NumCast,
    V: NumCast + std::ops::Sub<Output = V>,
{
    cast::<T, V>(n1).unwrap() - cast::<U, V>(n2).unwrap()
}

#[test]
fn test_int_div_ceil()
{
    assert_eq!(3, int_div_ceil(9, 3));
    assert_eq!(4, int_div_ceil(10, 3));
    assert_eq!(4, int_div_ceil(11, 3));
    assert_eq!(4, int_div_ceil(12, 3));
    assert_eq!(5, int_div_ceil(13, 3));
}

#[test]
fn test_int_sub_us()
{
    assert_eq!(19u8, int_sub_us(10u8, -9i8));

    assert_eq!(0u8, int_sub_us(10u8, 10i8));
}
