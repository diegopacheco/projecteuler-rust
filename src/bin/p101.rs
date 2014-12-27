//! [Problem 101](https://projecteuler.net/problem=101) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase, slicing_syntax)]

extern crate num;
#[phase(plugin, link)] extern crate common;
extern crate polynomial;

use std::iter::AdditiveIterator;
use num::rational::Ratio;
use num::{BigInt, Zero, One};
use polynomial::Polynomial;

fn u(n: BigInt) -> BigInt {
    let mut sum: BigInt = Zero::zero();
    let mut prod = One::one();
    for _ in range(0u, 11) {
        sum = sum + &prod;
        prod = &prod * (-&n);
    }
    sum
}

// Lagrange Interpolating with Naville's algorithm
fn op(ns: &[(BigInt, BigInt)]) -> Polynomial<BigInt> {
    let mut poly = Polynomial::new(vec![]);

    for (i, &(ref xi, ref yi)) in ns.iter().enumerate() {
        let mut term = Polynomial::new(vec![ Ratio::from_integer(yi.clone()) ]);

        for (j, &(ref xj, ref _yj)) in ns.iter().enumerate() {
            if i == j { continue }

            term = term * Polynomial::new(vec![Ratio::new(-xj, xi - xj),
                                               Ratio::new(One::one(), xi - xj)]);
        }
        poly = poly + term;
    }

    let data = poly.data().iter().map(Ratio::to_integer).collect();
    Polynomial::new(data)
}

fn bop(ns: &[(BigInt, BigInt)]) -> BigInt {
    op(ns).eval(FromPrimitive::from_uint(ns.len() + 1).unwrap())
}

fn u_to_vec(dim: uint, f: fn(BigInt) -> BigInt) -> Vec<(BigInt, BigInt)> {
    Vec::from_fn(dim + 1, |i| {
        let n: BigInt = FromPrimitive::from_uint(i + 1).unwrap();
        (n.clone(), f(n))
    })
}

fn solve() -> String {
    let un = u_to_vec(10, u);
    range(0, 10)
        .map(|i| bop(un[.. i + 1]))
        .sum()
        .to_string()
}

problem!("37076114526", solve);

#[cfg(test)]
mod tests {
    use num::BigInt;

    #[test]
    fn op() {
        fn u(n: BigInt) -> BigInt { &n * &n * &n }
        let un = super::u_to_vec(3, u);
        assert_eq!("1", super::op(un[..1]).pretty("n"));
        assert_eq!("-6+7*n", super::op(un[..2]).pretty("n"));
        assert_eq!("6-11*n+6*n^2", super::op(un[..3]).pretty("n"));
        assert_eq!("n^3", super::op(un[]).pretty("n"));
    }

    #[test]
    fn bop() {
        fn u(n: BigInt) -> BigInt { &n * &n * &n }
        let un = super::u_to_vec(3, u);
        assert_eq!(1, super::bop(un[..1]).to_int().unwrap());
        assert_eq!(15, super::bop(un[..2]).to_int().unwrap());
        assert_eq!(58, super::bop(un[..3]).to_int().unwrap());
    }
}
