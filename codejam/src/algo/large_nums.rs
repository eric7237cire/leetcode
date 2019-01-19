/* This function calculates (a to power b)%MOD */
/*pow(int a, int b, int MOD) {

    long x = 1, y = a;

    while (b > 0)
    {
        if (b % 2 == 1)
        {
            x = (x * y);

            if (x > MOD)
                x %= MOD;
        }

        y = (y * y);

        if (y > MOD)
            y %= MOD;

        b /= 2;
    }

    return x;

}



/*  Modular Multiplicative Inverse

    Using Euler's Theorem

    a^(phi(m)) = 1 (mod m)

    a^(-1) = a^(m-2) (mod m) */
public static long InverseEuler(int n, int MOD)
{
return pow(n,MOD-2,MOD);
}*/

use num_traits::Unsigned;
use std::ops::{Add, Mul, Rem};

///1 + 2^2 + 3^3 + n^n...
pub fn sum_sq_to_n(n: usize, modulo: usize) -> usize
{
    let mut a = n;
    let mut b = n + 1;
    let mut c = 2 * n + 1;
    if a % 2 == 0 {
        a /= 2;
    } else if b % 2 == 0 {
        b /= 2;
    } else {
        c /= 2;
    }

    if a % 3 == 0 {
        a /= 3;
    } else if b % 3 == 0 {
        b /= 3;
    } else {
        c /= 3;
    }
    mul_mod(mul_mod(a, b, modulo), c, modulo)
}
//1+2+...+n
pub fn sum_0_to_n(n: usize, modulo: usize) -> usize
{
    let mut a = n;
    let mut b = n + 1;
    if a % 2 == 0 {
        a /= 2;
    } else {
        assert!(b % 2 == 0);
        b /= 2;
    }
    mul_mod(a, b, modulo)
}

pub fn sub_mod<T>(a: T, b: T, modulo: T) -> T
where
    T: Add<Output = T> + Rem<Output = T> + Copy + Unsigned,
{
    (modulo + a - b) % modulo
}

pub fn mul_mod<T>(a: T, b: T, modulo: T) -> T
where
    T: Mul<Output = T> + Rem<Output = T> + Copy,
{
    ((a % modulo) * (b % modulo)) % modulo
}
