//! Rust crate associated with the article [`DSS21`].
//!
//! Provides an efficient function to compute the condition number of *V_n*, the Vandermonde matrix associated with the *n*th cyclotomic polynomial.
//! The condition number is computed via the trace of the matrix *H_n*, as shown in [`DSS20`].
//!
//! [`DSS20`]: https://doi.org/10.1515/jmc-2020-0009

use ndarray::{Array, ArrayBase, Dim, OwnedRepr};
use ndarray_linalg::*;
use num::integer::gcd;
use red_primality::{euler_totient, mobius};

#[inline]
fn ct(t: i64, n: u64) -> i64 {
    let g_nt = gcd(n as i64, t) as u64;
    mobius(n, g_nt) * (euler_totient(n) as i64) / (euler_totient(n / g_nt) as i64)
}

#[inline]
fn get_index(i: usize, j: usize) -> usize {
    let idx = (j as i64 - i as i64).abs();
    idx as usize
}

#[inline]
fn get_h(n: u64) -> ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> {
    let m = euler_totient(n) as usize;
    let v: Vec<_> = (0..(m as i64)).map(|t: i64| ct(t, n)).collect();
    let g = Array::from_shape_fn((m, m), |(i, j)| v[get_index(i, j)] as f64);
    let g_inv = g.inv_into().unwrap();

    (n as f64) * g_inv
}

/// Compute the trace of *H_n*.
///
/// *H_n* is defined as *n G_n^{-1}*, where *G_n* is the Gram matrix of *V_n*, the Vandermonde matrix associated with the *n*th cyclotomic polynomial.
///
/// In [`DSS20`] it is shown that for every positive integer *n*, the matrix *H_n* has integer entries.
///
/// [`DSS20`]: https://doi.org/10.1515/jmc-2020-0009
#[inline]
pub fn tr_h(n: u64) -> u64 {
    let h = get_h(n);
    let tr = h.trace().unwrap();

    tr.round() as u64
}

/// Compute the condition number of *V_n*.
///
/// *V_n* is the Vandermonde matrix associated with the *n*th cyclotomic polynomial.
///
/// As shown in [`DSS20`], the condition number can be computed as *m\sqrt(n) * sqrt(Tr(H_n))*.
/// The trace of *H_n* is computed in [`tr_h`].
///
/// [`DSS20`]: https://doi.org/10.1515/jmc-2020-0009
#[inline]
pub fn cond(n: u64) -> f64 {
    let tr = tr_h(n) as f64;
    let m = euler_totient(n) as f64;

    m * (tr / (n as f64)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tr_prime() {
        let primes: [u64; 5] = [11, 101, 103, 137, 179];
        let traces = primes.map(|p| tr_h(p));

        assert_eq!(traces, [20, 200, 204, 272, 356]);
    }

    #[test]
    fn tr_prime_power() {
        let primes_power: [u64; 5] = [25, 27, 49, 121, 841];
        let traces = primes_power.map(|p| tr_h(p));

        assert_eq!(traces, [40, 36, 84, 220, 1624]);
    }

    #[test]
    fn tr_squarefree() {
        let squarefrees: [u64; 5] = [259, 534, 649, 785, 901];
        let traces = squarefrees.map(|p| tr_h(p));

        assert_eq!(traces, [1576, 1640, 5776, 3740, 11600]);
    }

    macro_rules! precision_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                #[ignore]
                fn $name() {
                    let (n, tol) = $value;
                    let h = get_h(n);

                    for ((i, j), x) in h.indexed_iter() {
                        let fract = x.fract();
                        assert!(
                            fract < tol || (1f64 - fract) < tol,
                            "Entry ({}, {}): {} is above tollerance {}",
                            i,
                            j,
                            x,
                            tol
                        )
                    }
                }
            )*
        };
    }

    precision_tests! {
        precision_3: (3, 1e-9),
        precision_15: (15, 1e-9),
        precision_107: (107, 1e-9),
        precision_1155: (1155, 1e7),
        precision_8151: (8151, 1e-4),
    }
}
