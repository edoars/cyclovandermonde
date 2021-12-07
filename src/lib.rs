use ndarray::Array;
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
pub fn tr_h(n: u64) -> u64 {
    let m = euler_totient(n) as usize;
    let v: Vec<_> = (0..(m as i64)).map(|t: i64| ct(t, n)).collect();
    let g = Array::from_shape_fn((m, m), |(i, j)| v[get_index(i, j)] as f64);
    let g_inv = g.inv_into().unwrap();
    let tr = ((n as f64) * g_inv).trace().unwrap();

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
