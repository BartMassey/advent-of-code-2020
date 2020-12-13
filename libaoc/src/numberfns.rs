// Copyright © 2019 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Number-theoretic functions for Advent of Code solutions.

/// The GCD is not part of standard Rust. We don't need
/// super-efficiency, so we just use the faster form of the
/// [Euclidean
/// Algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm#Procedure).
#[allow(clippy::many_single_char_names)]
pub fn gcd(m: u64, n: u64) -> u64 {
    assert!(m > 0 && n > 0);
    let (mut a, mut b) = if m > n {
        (m, n)
    } else {
        (n, m)
    };
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[test]
fn test_gcd() {
    assert_eq!(1, gcd(3, 5));
    assert_eq!(2, gcd(2, 4));
    assert_eq!(2, gcd(4, 2));
    assert_eq!(3, gcd(9, 12));
}

/// The LCM of a pair of numbers is computed as their
/// product divided by their GCD.  The implementation is
/// careful to do things in optimal order to avoid overflow
/// when possible.
pub fn lcm(a: u64, b: u64) -> u64 {
    assert!(a > 0 && b > 0);
    let (mut p, q) = if a > b {
        (a, b)
    } else {
        (b, a)
    };
    p /= gcd(p, q);
    p * q
}

#[test]
fn test_lcm() {
    assert_eq!(12, lcm(12, 1));
    assert_eq!(12, lcm(1, 12));
    assert_eq!(12, lcm(4, 6));
    assert_eq!(60, lcm(20, 6));
    assert_eq!(100, lcm(25, 4));
}

/// Returns -1, 0 or 1 as the input is negative, zero or
/// positive.
pub fn sgn(x: i64) -> i64 {
    if x > 0 {
        return 1;
    }
    if x < 0 {
        return -1;
    }
    0
}
