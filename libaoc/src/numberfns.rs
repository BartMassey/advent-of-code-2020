// Copyright © 2019 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Number-theoretic functions for Advent of Code solutions.

use std::convert::TryFrom;

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

/// Extended Euclidean algorithm for GCD. Suitable for
/// finding modular inverses, etc.
/// 
/// From an unsigned-inputs [C++
/// implementation](https://jeffhurchalla.com/2018/10/13/implementing-the-extended-euclidean-algorithm-with-unsigned-inputs/)
/// by Jeff Hurchala.
pub fn extended_gcd(a: u64, b: u64) -> (u64, i64, i64) {
    let mut x1 = 1i64;
    let mut y1 = 0i64;
    let mut a1 = a;
    let mut x0 = 0i64;
    let mut y0 = 1i64;
    let mut a2 = b;
    let mut q = 0u64;

    while a2 != 0 {
        let x2 = x0 - q as i64 *x1;
        let y2 = y0 - q as i64 * y1;
        x0 = x1;
        y0 = y1;
        let a0 = a1;
        x1 = x2;
        y1 = y2;
        a1 = a2;
        q = a0 / a1;
        a2 = a0 - q*a1;
    }
    (a1, x1, y1)
}

#[test]
fn test_extended_gcd() {
    // https://www.hackerrank.com/contests/test-contest-47/challenges/m158-multiple-euclid
    assert_eq!(extended_gcd(3, 5), (1, 2, -1));
    // https://brilliant.org/wiki/extended-euclidean-algorithm/
    assert_eq!(extended_gcd(1914, 899), (29, 8, -17));
    // http://www.math.cmu.edu/~bkell/21110-2010s/extended-euclidean.html
    assert_eq!(extended_gcd(1398, 324), (6, -19, 82));
}

/// Modular multiplicative inverse. Returns `None` if the
/// operation is ill-defined.
/// 
/// From an unsigned-inputs [C++
/// implementation](https://jeffhurchalla.com/2018/10/13/implementing-the-extended-euclidean-algorithm-with-unsigned-inputs/)
/// by Jeff Hurchala.
pub fn mod_inv(modulus: u64, value: u64) -> Option<u64> {
    // Ordinarily, operations modulo 0 are undefined.
    if modulus == 0 {
        return None;
    }
    // Without this "if" clause, when `modulus == 1` and
    // `value == 1`, this function would calculate the
    // result to be 1.  That result wouldn't be completely
    // wrong, but it isn't reduced.  We always want a fully
    // reduced result.  When `modulus == 1`, the fully
    // reduced result will always be 0.
    if modulus == 1 {
        return Some(0);
    }
    let (gcd, _, y) = extended_gcd(modulus, value);
    if gcd != 1 {
        return None;
    }
    if y >= 0 {
        Some(y as u64)
    } else {
        Some((y + i64::try_from(modulus).unwrap()) as u64)
    }
}

#[test]
fn test_mod_inv() {
    // No unique inverse because inputs aren't
    // relatively prime.
    assert_eq!(mod_inv(9, 12), None);
    // https://rosettacode.org/wiki/Modular_inverse
    assert_eq!(mod_inv(2017, 42), Some(1969));
}

/// Solution *x* to a pair of congruences
///
/// > *x* ≡ *a* (mod *m*)  
/// > *x* ≡ *b* (mod *n*)
///
/// if one exists.
///
/// From
/// [Wikipedia](https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Generalization_to_non-coprime_moduli).
#[allow(clippy::many_single_char_names)]
pub fn crt(a: u64, b: u64, m: u64, n: u64) -> Option<u64> {
    let (g, u, v) = extended_gcd(m, n);
    if a % g == b % g {
        let x = a as i64 * v * n as i64
            + b as i64 * u * m as i64;
        let x = if x >= 0 {
            x
        } else {
            x + m as i64 * n as i64 / g as i64
        };
        assert!(x >= 0);
        Some(x as u64 / g)
    } else {
        None
    }
}

#[test]
fn test_crt() {
    assert_eq!(crt(3, 4, 5, 7), Some(18));
    assert_eq!(crt(3, 4, 5, 6), Some(28));
    assert_eq!(crt(3, 4, 6, 6), None);
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
